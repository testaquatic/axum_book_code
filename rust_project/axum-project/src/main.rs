use axum::{
    Json, Router,
    extract::{Multipart, Path},
    http::StatusCode,
    routing::get,
};
use axum_extra::{
    TypedHeader,
    headers::{ContentLength, ContentType},
};
use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route(
            "/",
            get(hello)
                .post(upload)
                .put(|| async move { "Updating" })
                .delete(|| async move { "■" }),
        )
        .route("/{num}", get(hello));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello(
    Path(num): Path<i32>,
) -> (
    TypedHeader<ContentType>,
    TypedHeader<ContentLength>,
    (StatusCode, Json<Value>),
) {
    match num {
        0 => (
            TypedHeader(ContentType::json()),
            TypedHeader(ContentLength(26)),
            (
                StatusCode::CREATED,
                Json(json!({"message": "Hello World!"})),
            ),
        ),
        _ => (
            TypedHeader(ContentType::json()),
            TypedHeader(ContentLength(20)),
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Error during creation"})),
            ),
        ),
    }
}

async fn upload(mut body: Multipart) -> String {
    if let Ok(Some(field)) = body.next_field().await {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        format!("{} : {} bytes", name, data.len())
    } else {
        "No field found in multipart data".to_string()
    }
}
