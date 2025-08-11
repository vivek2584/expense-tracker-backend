use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers::users::{login, logout, my_profile, register, update_user};

pub fn user_routes() -> Router {
    Router::new()
        .route("/users/register", post(register))
        .route("/users/login", post(login))
        .route("/users/logout", post(logout))
        .route("/users/me", get(my_profile).put(update_user))
}
