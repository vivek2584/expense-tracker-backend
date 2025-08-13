use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handlers::users::{login, my_profile, register, update_user},
    middlewares::GlobalAppState,
};

pub fn user_routes() -> Router<GlobalAppState> {
    Router::new()
        .route("/users/register", post(register))
        .route("/users/login", post(login))
        .route("/users/me", get(my_profile).patch(update_user))
}
