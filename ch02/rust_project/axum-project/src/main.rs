use axum::{
    Router,
    routing::{delete, get, post, put},
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async move { "Welcome to Axum!" }))
        .route("/", post(|| async move { "Post Something!" }))
        .route("/", put(|| async move { "Updating..." }))
        .route("/", delete(|| async move { "@" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
