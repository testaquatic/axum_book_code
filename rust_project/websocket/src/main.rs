use std::sync::Arc;

use axum::{
    Router,
    body::Body,
    extract::{
        State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    http::{HeaderMap, Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
};
use futures_util::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use tokio::sync::{
    Mutex,
    broadcast::{self, Receiver, Sender},
};

#[derive(Debug, Clone)]
struct AppState {
    broadcast_tx: Arc<Mutex<Sender<Message>>>,
}

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel(32);
    let app = AppState {
        broadcast_tx: Arc::new(Mutex::new(tx)),
    };

    let app = Router::new()
        .route("/ws", get(websocket_handler))
        .route_layer(middleware::from_fn(authenticate))
        .with_state(app);

    let listen = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listen, app).await.unwrap();
}

async fn websocket_handler(ws: WebSocketUpgrade, State(app): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, app))
}

async fn handle_socket(ws: WebSocket, app: AppState) {
    let (ws_tx, ws_rx) = ws.split();
    let ws_tx = Arc::new(Mutex::new(ws_tx));

    let broadcast_rx = app.broadcast_tx.lock().await.subscribe();
    tokio::spawn(async move {
        recv_broadcast(ws_tx, broadcast_rx).await;
    });

    recv_from_client(ws_rx, app.broadcast_tx).await;
}

async fn recv_broadcast(
    client_tx: Arc<Mutex<SplitSink<WebSocket, Message>>>,
    mut broadcast_rx: Receiver<Message>,
) {
    while let Ok(msg) = broadcast_rx.recv().await {
        if client_tx.lock().await.send(msg).await.is_err() {
            return;
        }
    }
}

async fn recv_from_client(
    mut client_rx: SplitStream<WebSocket>,
    broadcast_tx: Arc<Mutex<Sender<Message>>>,
) {
    while let Some(Ok(msg)) = client_rx.next().await {
        if matches!(msg, Message::Close(_)) {
            return;
        }
        if broadcast_tx.lock().await.send(msg).is_err() {
            return;
        }
    }
}

async fn authenticate(
    headers: HeaderMap,
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    if headers
        .get("Authorization")
        .map(|v| v == "Bearer roken")
        .unwrap_or(false)
    {
        Ok(next.run(request).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
