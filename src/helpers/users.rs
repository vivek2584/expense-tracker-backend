use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash,
};
use axum::http::StatusCode;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Serialize;

use crate::errors::GlobalAppError;

pub async fn hash_password(password: String) -> Result<String, GlobalAppError> {
    tokio::task::spawn_blocking(move || -> Result<String, GlobalAppError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon = Argon2::default();
        Ok(PasswordHash::generate(argon, password, salt.as_salt())
            .map_err(|_| {
                GlobalAppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "server_error".to_string(),
                )
            })?
            .to_string())
    })
    .await
    .unwrap()
}

pub async fn verify_password(
    password: String,
    hashed_password: String,
) -> Result<(), GlobalAppError> {
    tokio::task::spawn_blocking(move || -> Result<(), GlobalAppError> {
        PasswordHash::new(&hashed_password)
            .map_err(|_| {
                GlobalAppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "error parsing hashed password!".to_string(),
                )
            })?
            .verify_password(&[&Argon2::default()], password)
            .map_err(|err| match err {
                argon2::password_hash::Error::Password => {
                    GlobalAppError::new(StatusCode::BAD_REQUEST, "invalid password".to_string())
                }
                _ => GlobalAppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "password verification error!".to_string(),
                ),
            })
    })
    .await
    .unwrap()
}

#[derive(Serialize)]
struct Claims {
    sub: String,
    iat: i64,
    exp: i64,
}

pub fn create_jwt(uuid: String) -> Result<String, GlobalAppError> {
    let header = Header::new(jsonwebtoken::Algorithm::HS384);

    let now = Utc::now();
    let claims = Claims {
        sub: uuid,
        iat: now.timestamp(),
        exp: (now + Duration::hours(1)).timestamp(),
    };

    dotenvy::dotenv().unwrap();
    let hmac_key = dotenvy::var("HMAC_KEY").map_err(|_| {
        GlobalAppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "unable to read HMAC KEY from .env!".to_string(),
        )
    })?;
    let key = EncodingKey::from_base64_secret(hmac_key.as_str()).map_err(|_| {
        GlobalAppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "error parsing hmac key into encoded key type!".to_string(),
        )
    })?;

    encode(&header, &claims, &key).map_err(|_| {
        GlobalAppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "error creating jwt token!".to_string(),
        )
    })
}
