use std::sync::Arc;

use auth::session::store::Session;
use axum::{
    extract::{
        State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
};
use chrono::DateTime;
use futures::{SinkExt, StreamExt};
use scylla::value::CqlTimeuuid;
use serde_json::json;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{
    model::message::{ChatMessage, ChatMessageInput, MessagePayload},
    state::AppState,
};

pub async fn chat(
    ws: WebSocketUpgrade,
    State(app): State<Arc<AppState>>,
    session: Session,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, app, session))
}

async fn handle_socket(mut socket: WebSocket, app: Arc<AppState>, session: Session) {
    let chat_id = Uuid::parse_str("c271f73c-1c05-4d74-ba4a-d3a3840a7069").unwrap();

    let messages = app
        .db
        .query_unpaged(r#"SELECT * FROM ks.messages LIMIT 20"#, &[])
        .await
        .unwrap()
        .into_rows_result()
        .unwrap();
    let messages = messages
        .rows::<ChatMessage>()
        .unwrap()
        .map(|row| row.unwrap())
        .map(|message| MessagePayload {
            content: message.content,
            id: {
                let (s, ns) = message.id.as_ref().get_timestamp().unwrap().to_unix();
                DateTime::from_timestamp(s as i64, ns).unwrap().to_rfc3339()
            },
            user_id: message.user_id,
            chat_id: message.chat_id.to_string(),
        })
        .collect::<Vec<MessagePayload>>();
    socket
        .send(Message::text(
            json!({ "kind": "messages", "data": messages }).to_string(),
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

            if msg_contents.content.is_empty() {
                rx_sender
                    .lock()
                    .await
                    .send(Message::text(
                        json!({ "data": "Message cannot be empty.", "kind": "error" }).to_string(),
                    ))
                    .await
                    .unwrap();
                continue;
            }

            let timeuuid = CqlTimeuuid::from(Uuid::now_v1(
                &mac_address::get_mac_address().unwrap().unwrap().bytes(),
            ));

            let msg = ChatMessage {
                id: timeuuid,
                content: msg_contents.content.clone(),
                user_id: session.0.user_id.clone(),
                chat_id,
            };

            app_tx.channel_tx.send(msg.clone()).unwrap();
            app_tx
                .db
                .query_unpaged(
                    r#"INSERT INTO ks.messages (id, content, user_id, chat_id) VALUES (?, ?, ?, ?)"#,
                    msg,
                )
                .await
                .unwrap();
        }
    });

    let mut recv_task = tokio::spawn(async move {
        let mut rx = app.channel_tx.subscribe();
        while let Ok(message) = rx.recv().await {
            sender
                .lock()
                .await
                .send(Message::text(
                    json!({ "kind": "message", "data": MessagePayload { content: message.content, id: {
                      let (s, ns) = message.id.as_ref().get_timestamp().unwrap().to_unix();
                      DateTime::from_timestamp(s as i64, ns).unwrap().to_rfc3339()
                  }, user_id: message.user_id, chat_id: message.chat_id.to_string() } }).to_string(),
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
