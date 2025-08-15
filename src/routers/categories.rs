use axum::{middleware::from_fn_with_state, routing::post, Router};

use crate::{
    handlers::categories::{create_category, list_categories},
    middlewares::{auth::validate_jwt, GlobalAppState},
};

pub fn category_routes(state: GlobalAppState) -> Router<GlobalAppState> {
    Router::new()
        .route("/categories", post(create_category).get(list_categories))
        .route_layer(from_fn_with_state(state, validate_jwt))
}
