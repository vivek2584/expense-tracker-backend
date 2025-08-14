use axum::{middleware::from_fn_with_state, routing::post, Router};

use crate::{
    handlers::categories::create_category,
    middlewares::{auth::validate_jwt, GlobalAppState},
};

pub fn category_routes(state: GlobalAppState) -> Router<GlobalAppState> {
    Router::new()
        .route("/categories", post(create_category))
        .route_layer(from_fn_with_state(state, validate_jwt))
}
