use axum::{Router, extract::Multipart, routing::post};

async fn uplaod(mut body: Multipart) -> String {
    if let Ok(Some(field)) = body.next_field().await {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        format!("{} : {} bytes", name, data.len())
    } else {
        "No field found in multipart data".to_string()
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", post(uplaod));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
