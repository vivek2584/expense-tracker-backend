use axum::Router;
use sqlx::{Pool, Postgres};
mod users;

pub fn app_router(pool: Pool<Postgres>) -> Router {
    Router::new().merge(users::user_routes()).with_state(pool)
}
