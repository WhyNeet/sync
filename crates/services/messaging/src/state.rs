use std::env;

use common::events::Event;
use rdkafka::{
    ClientConfig, Message,
    consumer::{Consumer, StreamConsumer},
};
use scylla::client::session::Session;
use tokio::sync::broadcast::{self, Sender};

use crate::{model::message::ChatMessage, storage};

#[derive(Debug)]
pub struct AppState {
    pub db: Session,
    pub channel_tx: Sender<ChatMessage>,
    pub auth_event_rx: flume::Receiver<Event>,
}

impl AppState {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let session = storage::create_session().await?;

        storage::prepare_storage(&session).await?;

        let (channel_tx, _) = broadcast::channel(1);

        let (auth_event_tx, auth_event_rx) = flume::unbounded();
        let hosts = format!("{}", env::var("KAFKA_BROKER_URI").unwrap());
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", hosts)
            .set("group.id", "messaging")
            .set("security.protocol", "plaintext")
            .set("enable.auto.commit", "true")
            .create()?;

        consumer.subscribe(&["auth"]).unwrap();

        tokio::spawn(async move {
            while let Ok(message) = consumer.recv().await {
                let event =
                    serde_json::from_slice::<Event>(message.detach().payload().unwrap()).unwrap();
                auth_event_tx.send(event).unwrap();
            }
        });

        Ok(Self {
            db: session,
            channel_tx,
            auth_event_rx,
        })
    }
}
