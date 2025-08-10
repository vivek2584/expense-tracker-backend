use expense_tracker_backend::routers;

#[tokio::main]
async fn main() {
    let app = routers::app_router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
