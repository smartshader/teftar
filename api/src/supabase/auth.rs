use super::client::SupabaseClient;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: User,
}

#[derive(Debug, Deserialize)]
struct SupabaseAuthResponse {
    access_token: Option<String>,
    refresh_token: Option<String>,
    user: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct SupabaseErrorResponse {
    #[serde(default)]
    error_code: Option<String>,
    #[serde(default)]
    msg: Option<String>,
    #[serde(default)]
    error_description: Option<String>,
    #[serde(default)]
    message: Option<String>,
}

pub enum AuthError {
    EmailNotConfirmed,
    InvalidCredentials,
    InvalidToken,
    ExpiredToken,
    MissingData(String),
    NetworkError(String),
    UnknownError(String),
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::EmailNotConfirmed => write!(
                f,
                "Please confirm your email address before signing in. Check your inbox for the confirmation link."
            ),
            AuthError::InvalidCredentials => {
                write!(f, "Invalid email or password. Please try again.")
            }
            AuthError::InvalidToken => {
                write!(f, "The confirmation link is invalid. Please sign up again.")
            }
            AuthError::ExpiredToken => {
                write!(
                    f,
                    "The confirmation link has expired. Please sign up again to receive a new link."
                )
            }
            AuthError::MissingData(msg) => write!(f, "Missing data: {}", msg),
            AuthError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            AuthError::UnknownError(msg) => write!(f, "{}", msg),
        }
    }
}

impl SupabaseClient {
    pub async fn sign_up(
        &self,
        email: &str,
        password: &str,
    ) -> Result<Option<AuthResponse>, AuthError> {
        let site_url = env::var("SITE_URL").unwrap_or_else(|_| "http://localhost:5173".to_string());

        let body = serde_json::json!({
            "email": email,
            "password": password,
            "options": {
                "email_redirect_to": format!("{}/auth/callback", site_url)
            }
        });

        tracing::info!("POST /auth/signup - email: {}", email);

        let response = self
            .client()
            .post(self.auth_url("/signup"))
            .header("apikey", self.anon_key())
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| AuthError::NetworkError(e.to_string()))?;

        let status = response.status();

        if !status.is_success() {
            let error_response: SupabaseErrorResponse = response
                .json()
                .await
                .map_err(|e| AuthError::UnknownError(format!("Failed to parse error: {}", e)))?;

            tracing::error!("Signup failed for {}: {:?}", email, error_response);

            let error_msg = error_response
                .msg
                .or(error_response.error_description)
                .or(error_response.message)
                .unwrap_or_else(|| "Sign up failed".to_string());

            return Err(AuthError::UnknownError(error_msg));
        }

        let auth_response: SupabaseAuthResponse = response
            .json()
            .await
            .map_err(|e| AuthError::UnknownError(format!("Failed to parse response: {}", e)))?;

        // Check if tokens are present (auto-confirm enabled)
        if let (Some(access_token), Some(refresh_token), Some(user_data)) = (
            auth_response.access_token,
            auth_response.refresh_token,
            auth_response.user,
        ) {
            let user_id = user_data["id"]
                .as_str()
                .ok_or_else(|| AuthError::MissingData("user id".to_string()))?
                .to_string();

            let user_email = user_data["email"]
                .as_str()
                .ok_or_else(|| AuthError::MissingData("user email".to_string()))?
                .to_string();

            return Ok(Some(AuthResponse {
                access_token,
                refresh_token,
                user: User {
                    id: user_id,
                    email: user_email,
                },
            }));
        }

        // If no tokens, email confirmation is required
        Ok(None)
    }

    pub async fn sign_in(&self, email: &str, password: &str) -> Result<AuthResponse, AuthError> {
        let response = self
            .client()
            .post(self.auth_url("/token?grant_type=password"))
            .header("apikey", self.anon_key())
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "email": email,
                "password": password,
            }))
            .send()
            .await
            .map_err(|e| AuthError::NetworkError(e.to_string()))?;

        let status = response.status();

        if !status.is_success() {
            let error_response: SupabaseErrorResponse =
                response.json().await.unwrap_or(SupabaseErrorResponse {
                    error_code: None,
                    msg: None,
                    error_description: None,
                    message: None,
                });

            tracing::error!("Supabase signin error: {:?}", error_response);

            return Err(match error_response.error_code.as_deref() {
                Some("email_not_confirmed") => AuthError::EmailNotConfirmed,
                _ if status.as_u16() == 400 || status.as_u16() == 401 => {
                    AuthError::InvalidCredentials
                }
                _ => AuthError::UnknownError(
                    "Unable to sign in. Please try again later.".to_string(),
                ),
            });
        }

        let auth_response: SupabaseAuthResponse = response
            .json()
            .await
            .map_err(|e| AuthError::UnknownError(format!("Failed to parse response: {}", e)))?;

        let access_token = auth_response
            .access_token
            .ok_or_else(|| AuthError::MissingData("access_token".to_string()))?;

        let refresh_token = auth_response
            .refresh_token
            .ok_or_else(|| AuthError::MissingData("refresh_token".to_string()))?;

        let user_data = auth_response
            .user
            .ok_or_else(|| AuthError::MissingData("user".to_string()))?;

        let user_id = user_data["id"]
            .as_str()
            .ok_or_else(|| AuthError::MissingData("user id".to_string()))?
            .to_string();

        let user_email = user_data["email"]
            .as_str()
            .ok_or_else(|| AuthError::MissingData("user email".to_string()))?
            .to_string();

        Ok(AuthResponse {
            access_token,
            refresh_token,
            user: User {
                id: user_id,
                email: user_email,
            },
        })
    }

    pub async fn verify_otp(
        &self,
        token_hash: &str,
        verification_type: &str,
        email: Option<&str>,
    ) -> Result<AuthResponse, AuthError> {
        tracing::info!("POST /auth/verify-email - verifying OTP");

        let mut body = serde_json::json!({
            "token_hash": token_hash,
            "type": verification_type,
        });

        if let Some(email) = email {
            body["email"] = serde_json::json!(email);
        }

        let response = self
            .client()
            .post(self.auth_url("/verify"))
            .header("apikey", self.anon_key())
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| AuthError::NetworkError(e.to_string()))?;

        let status = response.status();

        if !status.is_success() {
            let error_response: SupabaseErrorResponse =
                response.json().await.unwrap_or(SupabaseErrorResponse {
                    error_code: None,
                    msg: None,
                    error_description: None,
                    message: None,
                });

            tracing::error!("OTP verification failed: {:?}", error_response);

            return Err(match error_response.error_code.as_deref() {
                Some("otp_expired") => AuthError::ExpiredToken,
                _ => AuthError::InvalidToken,
            });
        }

        let auth_response: SupabaseAuthResponse = response
            .json()
            .await
            .map_err(|e| AuthError::UnknownError(format!("Failed to parse response: {}", e)))?;

        let access_token = auth_response
            .access_token
            .ok_or_else(|| AuthError::MissingData("access_token".to_string()))?;

        let refresh_token = auth_response
            .refresh_token
            .ok_or_else(|| AuthError::MissingData("refresh_token".to_string()))?;

        let user_data = auth_response
            .user
            .ok_or_else(|| AuthError::MissingData("user".to_string()))?;

        let user_id = user_data["id"]
            .as_str()
            .ok_or_else(|| AuthError::MissingData("user id".to_string()))?
            .to_string();

        let user_email = user_data["email"]
            .as_str()
            .ok_or_else(|| AuthError::MissingData("user email".to_string()))?
            .to_string();

        Ok(AuthResponse {
            access_token,
            refresh_token,
            user: User {
                id: user_id,
                email: user_email,
            },
        })
    }
}
