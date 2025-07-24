mod api;
mod db;
mod entities;
mod utils;

use std::time::Duration;

use crate::{
    api::{
        auth::login,
        category::{delete_category, get_category, post_category},
        product::{delete_product, get_product, post_product, put_product},
        users::{delete_user, get_user, post_user, put_user},
    },
    db::init_db,
    utils::jwt::authenticate,
};
use axum::{
    Router, middleware,
    routing::{get, post},
};
use tower_http::{compression::CompressionLayer, timeout::TimeoutLayer, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::{
    EnvFilter,
    fmt::{self},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

pub async fn run() {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    info!("Connecting to DB...");
    let conn = init_db().await;

    info!("Starting server...");
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
        .route_layer(middleware::from_fn(authenticate))
        .route("/auth/login", post(login))
        .route("/auth/create", post(post_user))
        .with_state(conn)
        .layer(TimeoutLayer::new(Duration::from_millis(3000)))
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
