use std::sync::Arc;

use axum::{
    extract::{
        State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};

use crate::state::AppState;

pub async fn handle_client(
    ws: WebSocketUpgrade,
    State(app): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, app))
}

async fn handle_socket(socket: WebSocket, app: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();

    let app_tx = app.clone();

    let mut send_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            let bytes = msg.into_data();
            let msg_contents = String::from_utf8_lossy(&bytes).to_string();
            app_tx.channel_tx.send(msg_contents).unwrap();
        }
    });

    let mut recv_task = tokio::spawn(async move {
        let mut rx = app.channel_tx.subscribe();
        while let Ok(msg) = rx.recv().await {
            sender.send(Message::text(msg)).await.unwrap();
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
