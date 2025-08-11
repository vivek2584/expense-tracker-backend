use axum::Json;
use axum::{extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::query;
use sqlx::{prelude::FromRow, query_as, PgPool};

use crate::errors::GlobalAppError;
use crate::helpers::users::hash_password;

pub async fn register(
    State(pool): State<PgPool>,
    Json(register_data): Json<RegisterUserDetails>,
) -> Result<Json<UserDetails>, GlobalAppError> {
    let rows =
        query_as::<_, UserDetailRow>("SELECT name, email FROM users WHERE name = $1 OR email = $2")
            .bind(register_data.user_name.as_str())
            .bind(register_data.email.as_str())
            .fetch_all(&pool)
            .await
            .map_err(|_| {
                GlobalAppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "database error".to_string(),
                )
            })?;

    if !(rows.is_empty()) {
        Err(GlobalAppError::new(
            StatusCode::BAD_REQUEST,
            "username or email already exists, try again !".to_string(),
        ))
    } else {
        let password_hash = hash_password(register_data.password).await?;
        query("INSERT INTO users (name, email, password_hash, is_active) VALUES ($1, $2, $3, $4)")
            .bind(register_data.user_name.as_str())
            .bind(register_data.email.as_str())
            .bind(password_hash)
            .bind(true)
            .execute(&pool)
            .await
            .map_err(|_| {
                GlobalAppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "database error!".to_string(),
                )
            })?;

        Ok(Json(UserDetails {
            user_name: register_data.user_name,
            email: register_data.email,
            log_message: "User successfully registered".to_string(),
        }))
    }
}

pub async fn login() {}

pub async fn logout() {}

pub async fn my_profile() {}

pub async fn update_user() {}

#[derive(Deserialize)]
pub struct RegisterUserDetails {
    user_name: String,
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct UserDetails {
    user_name: String,
    email: String,
    log_message: String,
}

#[derive(FromRow)]
struct UserDetailRow {
    name: String,
    email: String,
}
