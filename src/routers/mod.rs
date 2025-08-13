use crate::middlewares::GlobalAppState;
use axum::Router;
mod users;

pub fn app_router(state: GlobalAppState) -> Router {
    Router::new()
        .merge(users::user_routes(state.clone()))
        .with_state(state)
}
