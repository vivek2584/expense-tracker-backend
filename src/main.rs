use expense_tracker_backend::{middlewares::GlobalAppState, routers};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    let database_url = dotenvy::var("DATABASE_URL").unwrap();
    let hmac_key = dotenvy::var("HMAC_KEY").unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(25)
        .connect(database_url.as_str())
        .await
        .unwrap();

    let app_state = GlobalAppState {
        pool,
        hmac: hmac_key,
    };

    let app = routers::app_router(app_state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
