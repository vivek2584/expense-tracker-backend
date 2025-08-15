use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    handlers::users::{delete_user, login, my_profile, register, update_password},
    middlewares::{GlobalAppState, auth::validate_jwt},
};

pub fn user_routes(state: GlobalAppState) -> Router<GlobalAppState> {
    Router::new()
        .route(
            "/users/me",
            get(my_profile).patch(update_password).delete(delete_user),
        )
        .route_layer(axum::middleware::from_fn_with_state(state, validate_jwt))
        .route("/users/register", post(register))
        .route("/users/login", post(login))
}
