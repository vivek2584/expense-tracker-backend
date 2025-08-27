use axum::{middleware::from_fn_with_state, routing::post, Router};

use crate::{
    handlers::transactions::add_expense,
    middlewares::{auth::validate_jwt, GlobalAppState},
};

pub fn transaction_routes(state: GlobalAppState) -> Router<GlobalAppState> {
    Router::new()
        .route("/transactions", post(add_expense))
        .route_layer(from_fn_with_state(state, validate_jwt))
}
