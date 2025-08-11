use axum::Router;
mod users;

pub fn app_router() -> Router {
    Router::new().merge(users::user_routes())
}
