use sqlx::PgPool;

pub mod auth;

#[derive(Clone)]
pub struct GlobalAppState {
    pub pool: PgPool,
    pub hmac: String,
}
