mod api;
mod db;
mod entities;
mod utils;

use crate::{
    api::{
        category::{delete_category, get_category, post_category},
        product::{delete_product, get_product, post_product, put_product},
        users::{delete_user, get_user, post_user, put_user},
    },
    db::init_db,
};
use axum::{Router, routing::get};

pub async fn run() {
    dotenvy::dotenv().ok();
    let conn = init_db().await;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    let app = Router::new()
        .route(
            "/users",
            get(get_user)
                .post(post_user)
                .put(put_user)
                .delete(delete_user),
        )
        .route(
            "/category",
            get(get_category)
                .post(post_category)
                .delete(delete_category),
        )
        .route(
            "/product",
            get(get_product)
                .post(post_product)
                .put(put_product)
                .delete(delete_product),
        )
        .with_state(conn);

    axum::serve(listener, app).await.unwrap();
}
