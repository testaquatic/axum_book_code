use axum::{Router, routing::get};
use module::{
    api::{
        category::{delete_category, get_category, post_category},
        users::{delete_user, get_user, post_user},
    },
    db::init_db,
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let conn = init_db().await;

    let app = Router::new()
        .route("/users", get(get_user).post(post_user).delete(delete_user))
        .route(
            "/category",
            get(get_category)
                .post(post_category)
                .delete(delete_category),
        )
        .with_state(conn);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .expect("Failed to bind TcpListener");

    axum::serve(listener, app).await.unwrap()
}
