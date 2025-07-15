use axum::{
    Router,
    extract::{FromRef, State},
    routing::get,
};

#[derive(FromRef, Clone)]
struct AppState {
    auth_token: String,
    current_user: i32,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        auth_token: "auth_token".to_string(),
        current_user: 3,
    };

    let app = Router::new()
        .route("/token", get(token))
        .route("/users", get(users))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn token(State(auth_token): State<String>) -> String {
    format!("Token: {}", auth_token)
}

async fn users(State(current_users): State<i32>) -> String {
    format!("Current user: {}", current_users)
}
