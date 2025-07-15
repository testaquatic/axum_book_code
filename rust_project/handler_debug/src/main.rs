use axum::{Router, debug_handler, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[debug_handler]
async fn handler() -> &'static str {
    "Hello, World!"
}
