use crate::middlewares::GlobalAppState;
use axum::Router;
mod categories;
mod users;

pub fn app_router(state: GlobalAppState) -> Router {
    Router::new()
        .merge(users::user_routes(state.clone()))
        .merge(categories::category_routes(state.clone()))
        .with_state(state)
}
