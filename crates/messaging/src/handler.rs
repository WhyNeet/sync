use std::sync::Arc;

use axum::{
    extract::{
        State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use serde_json::json;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{
    message::{ChatMessage, ChatMessageInput},
    state::AppState,
};

pub async fn handle_client(
    ws: WebSocketUpgrade,
    State(app): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, app))
}

async fn handle_socket(mut socket: WebSocket, app: Arc<AppState>) {
    let user_id = Uuid::new_v4().to_string();
    socket
        .send(Message::text(
            json!({ "kind": "auth", "data": user_id }).to_string(),
        ))
        .await
        .unwrap();

    let (sender, mut receiver) = socket.split();

    let sender = Arc::new(Mutex::new(sender));

    let app_tx = app.clone();

    let rx_sender = Arc::clone(&sender);
    let mut send_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            let bytes = msg.into_data();
            let msg_contents = String::from_utf8_lossy(&bytes).to_string();
            if msg_contents.is_empty() {
                break;
            }
            let msg_contents =
                if let Ok(msg) = serde_json::from_str::<ChatMessageInput>(&msg_contents) {
                    msg
                } else {
                    rx_sender
                        .lock()
                        .await
                        .send(Message::text(
                            json!({ "error": "Invalid message payload." }).to_string(),
                        ))
                        .await
                        .unwrap();
                    continue;
                };

            app_tx
                .channel_tx
                .send(ChatMessage {
                    content: msg_contents.content,
                    user_id: user_id.clone(),
                })
                .unwrap();
        }
    });

    let mut recv_task = tokio::spawn(async move {
        let mut rx = app.channel_tx.subscribe();
        while let Ok(msg) = rx.recv().await {
            sender
                .lock()
                .await
                .send(Message::text(
                    json!({ "kind": "message", "data": msg }).to_string(),
                ))
                .await
                .unwrap();
        }
    });

    tokio::select! {
      _ = &mut send_task => {
        recv_task.abort();
      },
      _ = &mut recv_task => {
        send_task.abort();
      }
    }
}
