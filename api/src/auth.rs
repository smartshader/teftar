use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize)]
pub struct SignUpRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub enum AuthError {
    BadRequest(String),
    Unauthorized(String),
    InternalError(String),
    ConfirmationRequired(String),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AuthError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AuthError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AuthError::ConfirmationRequired(msg) => (StatusCode::ACCEPTED, msg),
        };

        (status, Json(ErrorResponse { error: message })).into_response()
    }
}

pub async fn sign_up(Json(payload): Json<SignUpRequest>) -> Result<Json<AuthResponse>, AuthError> {
    let supabase_url = env::var("SUPABASE_URL")
        .map_err(|_| AuthError::InternalError("Missing SUPABASE_URL".to_string()))?;
    let supabase_anon_key = env::var("SUPABASE_ANON_KEY")
        .map_err(|_| AuthError::InternalError("Missing SUPABASE_ANON_KEY".to_string()))?;

    let site_url = env::var("SITE_URL").unwrap_or_else(|_| "http://localhost:5173".to_string());

    let client = reqwest::Client::new();

    let mut body = serde_json::json!({
        "email": payload.email,
        "password": payload.password,
    });

    // Add redirect URL if needed
    if let Some(obj) = body.as_object_mut() {
        obj.insert(
            "options".to_string(),
            serde_json::json!({
                "email_redirect_to": format!("{}/auth/callback", site_url)
            }),
        );
    }

    tracing::info!("POST /auth/signup - email: {}", payload.email);

    let response = client
        .post(format!("{}/auth/v1/signup", supabase_url))
        .header("apikey", &supabase_anon_key)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| AuthError::InternalError(format!("Request failed: {}", e)))?;

    let status = response.status();

    let auth_response: serde_json::Value = response
        .json()
        .await
        .map_err(|e| AuthError::InternalError(format!("Failed to parse response: {}", e)))?;

    if !status.is_success() {
        tracing::error!("Signup failed for {}: {:?}", payload.email, auth_response);
        let error_msg = auth_response["msg"]
            .as_str()
            .or(auth_response["error_description"].as_str())
            .or(auth_response["message"].as_str())
            .unwrap_or("Sign up failed");
        return Err(AuthError::BadRequest(error_msg.to_string()));
    }

    // Check if tokens are present (auto-confirm enabled)
    if let Some(access_token) = auth_response["access_token"].as_str() {
        let refresh_token = auth_response["refresh_token"]
            .as_str()
            .ok_or_else(|| AuthError::InternalError("Missing refresh_token".to_string()))?
            .to_string();

        let user_id = auth_response["user"]["id"]
            .as_str()
            .ok_or_else(|| AuthError::InternalError("Missing user id".to_string()))?
            .to_string();

        let user_email = auth_response["user"]["email"]
            .as_str()
            .ok_or_else(|| AuthError::InternalError("Missing user email".to_string()))?
            .to_string();

        return Ok(Json(AuthResponse {
            access_token: access_token.to_string(),
            refresh_token,
            user: User {
                id: user_id,
                email: user_email,
            },
        }));
    }

    // If no tokens, email confirmation is required
    // Return 202 Accepted with confirmation message
    Err(AuthError::ConfirmationRequired(
        "Account created! Please check your email to confirm your account before signing in."
            .to_string(),
    ))
}

pub async fn sign_in(Json(payload): Json<SignInRequest>) -> Result<Json<AuthResponse>, AuthError> {
    let supabase_url = env::var("SUPABASE_URL")
        .map_err(|_| AuthError::InternalError("Missing SUPABASE_URL".to_string()))?;
    let supabase_anon_key = env::var("SUPABASE_ANON_KEY")
        .map_err(|_| AuthError::InternalError("Missing SUPABASE_ANON_KEY".to_string()))?;

    let client = reqwest::Client::new();
    let response = client
        .post(format!(
            "{}/auth/v1/token?grant_type=password",
            supabase_url
        ))
        .header("apikey", &supabase_anon_key)
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "email": payload.email,
            "password": payload.password,
        }))
        .send()
        .await
        .map_err(|e| AuthError::InternalError(format!("Request failed: {}", e)))?;

    let status = response.status();

    if !status.is_success() {
        // Try to parse error response, but provide user-friendly message
        let error_response: serde_json::Value = response.json().await.unwrap_or_default();

        tracing::error!("Supabase signin error: {:?}", error_response);

        // Return user-friendly error message
        let error_msg = if status.as_u16() == 400 || status.as_u16() == 401 {
            "Invalid email or password. Please try again."
        } else {
            "Unable to sign in. Please try again later."
        };

        return Err(AuthError::Unauthorized(error_msg.to_string()));
    }

    let auth_response: serde_json::Value = response
        .json()
        .await
        .map_err(|e| AuthError::InternalError(format!("Failed to parse response: {}", e)))?;

    let access_token = auth_response["access_token"]
        .as_str()
        .ok_or_else(|| AuthError::InternalError("Missing access_token".to_string()))?
        .to_string();

    let refresh_token = auth_response["refresh_token"]
        .as_str()
        .ok_or_else(|| AuthError::InternalError("Missing refresh_token".to_string()))?
        .to_string();

    let user_id = auth_response["user"]["id"]
        .as_str()
        .ok_or_else(|| AuthError::InternalError("Missing user id".to_string()))?
        .to_string();

    let user_email = auth_response["user"]["email"]
        .as_str()
        .ok_or_else(|| AuthError::InternalError("Missing user email".to_string()))?
        .to_string();

    Ok(Json(AuthResponse {
        access_token,
        refresh_token,
        user: User {
            id: user_id,
            email: user_email,
        },
    }))
}
