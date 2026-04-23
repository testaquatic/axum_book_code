use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{Json, Router, body::Bytes, extract::State, http::StatusCode, routing::post};
use reqwest::Client;

type Cache = Arc<Mutex<HashMap<String, Bytes>>>;

#[derive(serde::Deserialize)]
struct Data {
    breed: String,
    num_pics: Option<i32>,
}

async fn proxy_handler(State(state): State<Cache>, Json(data): Json<Data>) -> (StatusCode, Bytes) {
    if let Some(body) = state.lock().unwrap().get(&data.breed) {
        println!("{} 캐시 히트", data.breed);
        return (StatusCode::OK, body.clone());
    }

    println!("{} 캐시 미스", data.breed);

    let url = format!(
        "https://dog.ceo/api/breed/{}/images/random{}",
        &data.breed,
        &data
            .num_pics
            .map(|num| format!("/{}", num))
            .unwrap_or(String::new())
    );

    let client = Client::new();
    let res = client.get(url).send().await.unwrap();

    let code = res.status().as_u16();
    let body = res.bytes().await.unwrap();
    let mut cache = state.lock().unwrap();
    cache.insert(data.breed, body.clone());

    (StatusCode::from_u16(code).unwrap(), body)
}

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(HashMap::<String, Bytes>::new()));
    let app = Router::new()
        .route("/", post(proxy_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
