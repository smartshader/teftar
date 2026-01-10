use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
pub enum ClientType {
    Company,
    Person,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PhoneNumber {
    #[serde(rename = "type")]
    pub phone_type: String,
    pub number: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Client {
    pub id: Uuid,
    pub user_id: Uuid,
    pub client_type: ClientType,
    pub company_name: Option<String>,
    pub person_name: Option<String>,
    pub email: Option<String>,
    pub phone_numbers: Value,
    pub country: Option<String>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub postal_code: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateClientRequest {
    pub client_type: ClientType,
    pub company_name: Option<String>,
    pub person_name: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub phone_numbers: Option<Vec<PhoneNumber>>,
    pub country: Option<String>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub postal_code: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateClientRequest {
    pub client_type: Option<ClientType>,
    pub company_name: Option<String>,
    pub person_name: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub phone_numbers: Option<Vec<PhoneNumber>>,
    pub country: Option<String>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub postal_code: Option<String>,
}

pub async fn list_clients(
    State(pool): State<PgPool>,
    user_id: Uuid,
) -> Result<Json<Vec<Client>>, (StatusCode, Json<Value>)> {
    let clients = sqlx::query_as::<_, Client>(
        "SELECT * FROM clients WHERE user_id = $1 ORDER BY created_at DESC",
    )
    .bind(user_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch clients: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Failed to fetch clients" })),
        )
    })?;

    Ok(Json(clients))
}

pub async fn create_client(
    State(pool): State<PgPool>,
    user_id: Uuid,
    Json(req): Json<CreateClientRequest>,
) -> Result<(StatusCode, Json<Client>), (StatusCode, Json<Value>)> {
    req.validate().map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": format!("Validation error: {}", e) })),
        )
    })?;

    let phone_numbers_json =
        serde_json::to_value(req.phone_numbers.unwrap_or_default()).map_err(|e| {
            tracing::error!("Failed to serialize phone numbers: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Invalid phone numbers format" })),
            )
        })?;

    let client = sqlx::query_as::<_, Client>(
        r#"
        INSERT INTO clients (
            user_id, client_type, company_name, person_name, email,
            phone_numbers, country, address_line1, address_line2,
            city, province, postal_code
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        RETURNING *
        "#,
    )
    .bind(user_id)
    .bind(req.client_type)
    .bind(req.company_name)
    .bind(req.person_name)
    .bind(req.email)
    .bind(phone_numbers_json)
    .bind(req.country)
    .bind(req.address_line1)
    .bind(req.address_line2)
    .bind(req.city)
    .bind(req.province)
    .bind(req.postal_code)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create client: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Failed to create client" })),
        )
    })?;

    Ok((StatusCode::CREATED, Json(client)))
}

pub async fn get_client(
    State(pool): State<PgPool>,
    user_id: Uuid,
    Path(id): Path<Uuid>,
) -> Result<Json<Client>, (StatusCode, Json<Value>)> {
    let client =
        sqlx::query_as::<_, Client>("SELECT * FROM clients WHERE id = $1 AND user_id = $2")
            .bind(id)
            .bind(user_id)
            .fetch_optional(&pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to fetch client: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "Failed to fetch client" })),
                )
            })?
            .ok_or_else(|| {
                (
                    StatusCode::NOT_FOUND,
                    Json(json!({ "error": "Client not found" })),
                )
            })?;

    Ok(Json(client))
}

pub async fn update_client(
    State(pool): State<PgPool>,
    user_id: Uuid,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateClientRequest>,
) -> Result<Json<Client>, (StatusCode, Json<Value>)> {
    req.validate().map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": format!("Validation error: {}", e) })),
        )
    })?;

    let _existing = sqlx::query("SELECT id FROM clients WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(user_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to check client: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to update client" })),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Client not found" })),
            )
        })?;

    let phone_numbers_json = if let Some(phones) = req.phone_numbers {
        serde_json::to_value(phones).map_err(|e| {
            tracing::error!("Failed to serialize phone numbers: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Invalid phone numbers format" })),
            )
        })?
    } else {
        sqlx::query_scalar::<_, Value>("SELECT phone_numbers FROM clients WHERE id = $1")
            .bind(id)
            .fetch_one(&pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to fetch existing phone numbers: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "Failed to update client" })),
                )
            })?
    };

    let client = sqlx::query_as::<_, Client>(
        r#"
        UPDATE clients
        SET
            client_type = COALESCE($1, client_type),
            company_name = COALESCE($2, company_name),
            person_name = COALESCE($3, person_name),
            email = COALESCE($4, email),
            phone_numbers = $5,
            country = COALESCE($6, country),
            address_line1 = COALESCE($7, address_line1),
            address_line2 = COALESCE($8, address_line2),
            city = COALESCE($9, city),
            province = COALESCE($10, province),
            postal_code = COALESCE($11, postal_code),
            updated_at = NOW()
        WHERE id = $12 AND user_id = $13
        RETURNING *
        "#,
    )
    .bind(req.client_type)
    .bind(req.company_name)
    .bind(req.person_name)
    .bind(req.email)
    .bind(phone_numbers_json)
    .bind(req.country)
    .bind(req.address_line1)
    .bind(req.address_line2)
    .bind(req.city)
    .bind(req.province)
    .bind(req.postal_code)
    .bind(id)
    .bind(user_id)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to update client: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Failed to update client" })),
        )
    })?;

    Ok(Json(client))
}

pub async fn delete_client(
    State(pool): State<PgPool>,
    user_id: Uuid,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<Value>)> {
    let result = sqlx::query("DELETE FROM clients WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(user_id)
        .execute(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete client: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to delete client" })),
            )
        })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Client not found" })),
        ));
    }

    Ok(StatusCode::NO_CONTENT)
}
