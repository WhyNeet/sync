use std::{env, sync::Arc, time::Duration};

use common::events::Event;
use rdkafka::{
    ClientConfig,
    producer::{FutureProducer, FutureRecord},
    util::Timeout,
};
use scylla::client::session::Session;

use crate::storage;

#[derive(Debug)]
pub struct AppState {
    pub db: Arc<Session>,
    pub bus_producer: flume::Sender<Event>,
}

impl AppState {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let session = storage::create_session().await?;

        storage::prepare_storage(&session).await?;

        let (tx, rx) = flume::unbounded();
        let hosts = format!("{}", env::var("KAFKA_BROKER_URI").unwrap());
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", hosts)
            .set("security.protocol", "plaintext")
            .set("message.timeout.ms", "5000")
            .create()?;

        tokio::spawn(async move {
            while let Ok(event) = rx.recv_async().await {
                producer
                    .send(
                        FutureRecord {
                            topic: "auth",
                            headers: None,
                            key: Some(&()),
                            partition: None,
                            timestamp: None,
                            payload: Some(&serde_json::to_string(&event).unwrap()),
                        },
                        Timeout::After(Duration::from_secs(10)),
                    )
                    .await
                    .unwrap();
            }
        });

        Ok(Self {
            db: Arc::new(session),
            bus_producer: tx,
        })
    }
}
