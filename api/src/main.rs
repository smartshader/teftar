mod auth;
mod clients;
mod middleware;
mod supabase;

use axum::{
    Json, Router,
    extract::Extension,
    middleware as axum_middleware,
    routing::{delete, get, post, put},
};
use serde_json::{Value, json};
use sqlx::PgPool;
use std::env;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let _guard = init_sentry();

    let sentry_layer = sentry_tracing::layer();

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .finish()
        .with(sentry_layer)
        .init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    tracing::info!("Database connection established");

    // Configure CORS based on environment
    let cors = if let Ok(allowed_origins) = env::var("ALLOWED_ORIGINS") {
        // Production: Use specific allowed origins
        let origins = allowed_origins
            .split(',')
            .map(|s| s.trim().parse().expect("Invalid origin URL"))
            .collect::<Vec<_>>();

        tracing::info!("CORS configured with allowed origins: {:?}", origins);

        CorsLayer::new()
            .allow_origin(origins)
            .allow_methods(Any)
            .allow_headers([
                axum::http::header::CONTENT_TYPE,
                axum::http::header::AUTHORIZATION,
            ])
            .allow_credentials(true)
    } else {
        // Development: Allow any origin
        tracing::warn!("CORS configured in permissive mode (development)");
        CorsLayer::permissive()
    };

    let protected_routes = Router::new()
        .route(
            "/clients",
            get(list_clients_handler).post(create_client_handler),
        )
        .route(
            "/clients/{id}",
            get(get_client_handler)
                .put(update_client_handler)
                .delete(delete_client_handler),
        )
        .layer(axum_middleware::from_fn_with_state(
            pool.clone(),
            middleware::auth_middleware,
        ));

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/auth/signup", post(auth::sign_up))
        .route("/auth/signin", post(auth::sign_in))
        .route("/auth/verify-email", post(auth::verify_email))
        .merge(protected_routes)
        .with_state(pool)
        .layer(sentry_tower::NewSentryLayer::new_from_top())
        .layer(sentry_tower::SentryHttpLayer::with_transaction())
        .layer(cors);

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}

async fn root() -> Json<Value> {
    Json(json!({
        "message": "Teftar API",
        "version": "0.1.0",
        "status": "running"
    }))
}

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy"
    }))
}

fn init_sentry() -> sentry::ClientInitGuard {
    let dsn = env::var("SENTRY_DSN").ok();
    let environment = env::var("SENTRY_ENVIRONMENT").unwrap_or_else(|_| "development".to_string());

    if dsn.is_none() {
        eprintln!("Sentry DSN not configured. Error tracking disabled.");
    }

    sentry::init((
        dsn,
        sentry::ClientOptions {
            release: sentry::release_name!(),
            environment: Some(environment.into()),
            sample_rate: if cfg!(debug_assertions) { 1.0 } else { 0.1 },
            traces_sample_rate: if cfg!(debug_assertions) { 1.0 } else { 0.1 },
            attach_stacktrace: true,
            send_default_pii: false,
            ..Default::default()
        },
    ))
}

async fn list_clients_handler(
    axum::extract::State(pool): axum::extract::State<PgPool>,
    Extension(user_id): Extension<Uuid>,
) -> Result<Json<Vec<clients::Client>>, (axum::http::StatusCode, Json<Value>)> {
    clients::list_clients(axum::extract::State(pool), user_id).await
}

async fn create_client_handler(
    axum::extract::State(pool): axum::extract::State<PgPool>,
    Extension(user_id): Extension<Uuid>,
    Json(req): Json<clients::CreateClientRequest>,
) -> Result<(axum::http::StatusCode, Json<clients::Client>), (axum::http::StatusCode, Json<Value>)>
{
    clients::create_client(axum::extract::State(pool), user_id, Json(req)).await
}

async fn get_client_handler(
    axum::extract::State(pool): axum::extract::State<PgPool>,
    Extension(user_id): Extension<Uuid>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> Result<Json<clients::Client>, (axum::http::StatusCode, Json<Value>)> {
    clients::get_client(axum::extract::State(pool), user_id, axum::extract::Path(id)).await
}

async fn update_client_handler(
    axum::extract::State(pool): axum::extract::State<PgPool>,
    Extension(user_id): Extension<Uuid>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
    Json(req): Json<clients::UpdateClientRequest>,
) -> Result<Json<clients::Client>, (axum::http::StatusCode, Json<Value>)> {
    clients::update_client(
        axum::extract::State(pool),
        user_id,
        axum::extract::Path(id),
        Json(req),
    )
    .await
}

async fn delete_client_handler(
    axum::extract::State(pool): axum::extract::State<PgPool>,
    Extension(user_id): Extension<Uuid>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> Result<axum::http::StatusCode, (axum::http::StatusCode, Json<Value>)> {
    clients::delete_client(axum::extract::State(pool), user_id, axum::extract::Path(id)).await
}
