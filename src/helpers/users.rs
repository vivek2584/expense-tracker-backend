use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash,
};
use axum::http::StatusCode;

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
