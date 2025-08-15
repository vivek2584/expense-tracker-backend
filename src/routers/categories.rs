use axum::{
    middleware::from_fn_with_state,
    routing::{delete, post},
    Router,
};

use crate::{
    handlers::categories::{create_category, delete_category, list_categories},
    middlewares::{auth::validate_jwt, GlobalAppState},
};

pub fn category_routes(state: GlobalAppState) -> Router<GlobalAppState> {
    Router::new()
        .route("/categories", post(create_category).get(list_categories))
        .route("/categories/{id}", delete(delete_category))
        .route_layer(from_fn_with_state(state, validate_jwt))
}
