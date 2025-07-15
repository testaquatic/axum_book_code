use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{Json, Router, body::Bytes, extract::State, http::StatusCode, routing::post};
use reqwest::Client;
use serde::Deserialize;

type Cache<T, U> = Arc<Mutex<HashMap<T, U>>>;

#[derive(Deserialize)]
struct Data {
    /// 종류
    breed: String,
    /// 사진 개수 옵션
    num_pics: Option<i32>,
}

async fn proxy_handler(
    State(state): State<Cache<String, Bytes>>,
    Json(data): Json<Data>,
) -> (StatusCode, Bytes) {
    if let Some(body) = state.lock().unwrap().get(&data.breed).cloned() {
        println!("{} 캐시 히트", &data.breed);
        return (StatusCode::OK, body);
    }

    println!("{} 캐시 미스", &data.breed);

    let mut url = format!("https://dog.ceo/api/breed/{}/images/random", data.breed);
    if let Some(num_pics) = data.num_pics {
        url.push_str(&format!("/{}", num_pics));
    }

    // 백엔드 서버에 요청
    let client = Client::new();
    let res = client.get(&url).send().await.unwrap();

    // 프록시 응답 리턴
    let code = res.status();
    let body = res.bytes().await.unwrap();
    let mut cache = state.lock().unwrap();
    cache.insert(data.breed, body.clone());

    (code, body)
}

#[tokio::main]
async fn main() {
    let state = Cache::default();
    let app = Router::new()
        .route("/", post(proxy_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
