use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{Json, Response},
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use std::env;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub exp: usize,
}

pub async fn auth_middleware(
    State(_pool): State<PgPool>,
    mut req: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "Missing authorization header" })),
            )
        })?;

    let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Invalid authorization header format" })),
        )
    })?;

    let jwk_json = env::var("SUPABASE_JWT_JWK").map_err(|_| {
        tracing::error!("SUPABASE_JWT_JWK not set");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Server configuration error" })),
        )
    })?;

    // Supabase uses ES256 algorithm with JWK format
    let mut validation = Validation::new(Algorithm::ES256);
    validation.set_audience(&["authenticated"]);

    let jwk: serde_json::Value = serde_json::from_str(&jwk_json).map_err(|e| {
        tracing::error!("Failed to parse JWK: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Server configuration error" })),
        )
    })?;

    let decoding_key = DecodingKey::from_ec_components(
        jwk["x"].as_str().unwrap_or(""),
        jwk["y"].as_str().unwrap_or(""),
    )
    .map_err(|e| {
        tracing::error!("Failed to create decoding key from JWK components: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Server configuration error" })),
        )
    })?;

    let token_data = decode::<Claims>(token, &decoding_key, &validation).map_err(|e| {
        tracing::warn!("JWT validation failed: {}", e);
        (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Invalid or expired token" })),
        )
    })?;

    let user_id = Uuid::parse_str(&token_data.claims.sub).map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Invalid user ID in token" })),
        )
    })?;

    req.extensions_mut().insert(user_id);

    Ok(next.run(req).await)
}
