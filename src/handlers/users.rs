use axum::Json;
use axum::{extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::query;
use sqlx::{prelude::FromRow, query_as};
use uuid::Uuid;

use crate::errors::GlobalAppError;
use crate::helpers::users::{create_jwt, hash_password, verify_password};
use crate::middlewares::GlobalAppState;

pub async fn register(
    State(state): State<GlobalAppState>,
    Json(register_data): Json<RegisterUserDetails>,
) -> Result<Json<RegisterResponseUserDetails>, GlobalAppError> {
    let rows =
        query_as::<_, UserDetailRow>("SELECT name, email FROM users WHERE name = $1 OR email = $2")
            .bind(register_data.user_name.as_str())
            .bind(register_data.email.as_str())
            .fetch_all(&state.pool)
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
            .execute(&state.pool)
            .await
            .map_err(|_| {
                GlobalAppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "database error!".to_string(),
                )
            })?;

        Ok(Json(RegisterResponseUserDetails {
            user_name: register_data.user_name,
            email: register_data.email,
            log_message: "User successfully registered".to_string(),
            token: None,
        }))
    }
}

pub async fn login(
    State(state): State<GlobalAppState>,
    Json(login_data): Json<LoginUserDetails>,
) -> Result<Json<LoginResponseUserDetails>, GlobalAppError> {
    let row =
        query_as::<_, UserPasswordRow>("SELECT id, name, password_hash FROM users WHERE name = $1")
            .bind(login_data.user_name.as_str())
            .fetch_one(&state.pool)
            .await
            .map_err(|error| match error {
                sqlx::Error::RowNotFound => GlobalAppError::new(
                    StatusCode::BAD_REQUEST,
                    "user not found!, please register and try again".to_string(),
                ),
                _ => {
                    eprintln!("--> DATABASE ERROR: {:?}", error);
                    GlobalAppError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "database error!".to_string(),
                    )
                }
            })?;

    let hashed_password = row.password_hash;
    verify_password(login_data.password, hashed_password).await?;

    let jwt_token = create_jwt(row.id.to_string(), state.hmac)?;

    Ok(Json(LoginResponseUserDetails {
        user_name: login_data.user_name,
        log_message: "successfully logged in!, jwt token expires in 1 hour".to_string(),
        token: Some(jwt_token),
    }))
}

pub async fn my_profile() {}

pub async fn update_user() {}

#[derive(Deserialize)]
pub struct RegisterUserDetails {
    user_name: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct LoginUserDetails {
    user_name: String,
    password: String,
}

#[derive(Serialize)]
pub struct RegisterResponseUserDetails {
    user_name: String,
    email: String,
    log_message: String,
    token: Option<String>,
}

#[derive(Serialize)]
pub struct LoginResponseUserDetails {
    user_name: String,
    log_message: String,
    token: Option<String>,
}

#[derive(FromRow)]
struct UserDetailRow {
    name: String,
    email: String,
}

#[derive(FromRow)]
struct UserPasswordRow {
    id: Uuid,
    name: String,
    password_hash: String,
}
