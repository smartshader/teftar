use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

use crate::supabase::auth::AuthError as SupabaseAuthError;
use crate::supabase::{AuthResponse, SupabaseClient};

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

#[derive(Debug, Deserialize)]
pub struct VerifyEmailRequest {
    pub token: String,
    pub type_: String,
    pub email: Option<String>,
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

impl From<SupabaseAuthError> for AuthError {
    fn from(err: SupabaseAuthError) -> Self {
        match err {
            SupabaseAuthError::EmailNotConfirmed => AuthError::Unauthorized(err.to_string()),
            SupabaseAuthError::InvalidCredentials => AuthError::Unauthorized(err.to_string()),
            SupabaseAuthError::InvalidToken => AuthError::Unauthorized(err.to_string()),
            SupabaseAuthError::ExpiredToken => AuthError::Unauthorized(err.to_string()),
            SupabaseAuthError::MissingData(msg) => AuthError::InternalError(msg),
            SupabaseAuthError::NetworkError(msg) => AuthError::InternalError(msg),
            SupabaseAuthError::UnknownError(msg) => AuthError::BadRequest(msg),
        }
    }
}

pub async fn sign_up(Json(payload): Json<SignUpRequest>) -> Result<Json<AuthResponse>, AuthError> {
    let client = SupabaseClient::new().map_err(|e| AuthError::InternalError(e))?;

    match client.sign_up(&payload.email, &payload.password).await? {
        Some(auth_response) => Ok(Json(auth_response)),
        None => Err(AuthError::ConfirmationRequired(
            "Account created! Please check your email to confirm your account before signing in."
                .to_string(),
        )),
    }
}

pub async fn sign_in(Json(payload): Json<SignInRequest>) -> Result<Json<AuthResponse>, AuthError> {
    let client = SupabaseClient::new().map_err(|e| AuthError::InternalError(e))?;

    let auth_response = client.sign_in(&payload.email, &payload.password).await?;

    Ok(Json(auth_response))
}

pub async fn verify_email(
    Json(payload): Json<VerifyEmailRequest>,
) -> Result<Json<AuthResponse>, AuthError> {
    let client = SupabaseClient::new().map_err(|e| AuthError::InternalError(e))?;

    let auth_response = client
        .verify_otp(&payload.token, &payload.type_, payload.email.as_deref())
        .await?;

    Ok(Json(auth_response))
}
