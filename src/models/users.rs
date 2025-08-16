use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RegisterUserDetails {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginUserDetails {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct ResponseUserDetails {
    pub username: String,
    pub email: String,
    pub log_message: String,
    pub token: Option<String>,
}

#[derive(Serialize)]
pub struct LoginResponseUserDetails {
    pub username: String,
    pub log_message: String,
    pub token: Option<String>,
}

#[derive(FromRow)]
pub struct UserDetailRow {
    pub name: String,
    pub email: String,
}

#[derive(FromRow)]
pub struct UserPasswordRow {
    pub id: Uuid,
    pub name: String,
    pub password_hash: String,
}

#[derive(FromRow, Serialize)]
pub struct UserProfileDetails {
    pub id: Uuid,
    pub name: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
    pub email: String,
    pub is_active: bool,
}

#[derive(Deserialize)]
pub struct PasswordPatch {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Deserialize)]
pub struct Password {
    pub password: String,
}

#[derive(FromRow)]
pub struct HashPassword {
    pub password_hash: String,
}
