mod auth;
mod supabase;

use axum::{
    Json, Router,
    routing::{get, post},
};
use serde_json::{Value, json};
use std::env;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

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
            .allow_headers(Any)
            .allow_credentials(true)
    } else {
        // Development: Allow any origin
        tracing::warn!("CORS configured in permissive mode (development)");
        CorsLayer::permissive()
    };

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/auth/signup", post(auth::sign_up))
        .route("/auth/signin", post(auth::sign_in))
        .route("/auth/verify-email", post(auth::verify_email))
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
