use std::env;

use scylla::client::{session::Session, session_builder::SessionBuilder};

pub async fn create_session() -> Result<Session, Box<dyn std::error::Error>> {
    let uri = env::var("SCYLLA_URI")?;
    let session = SessionBuilder::new().known_node(uri).build().await?;

    Ok(session)
}

pub async fn prepare_storage(session: &Session) -> Result<(), Box<dyn std::error::Error>> {
    session.query_unpaged(r#"
      CREATE KEYSPACE IF NOT EXISTS ks WITH REPLICATION = { 'class': 'NetworkTopologyStrategy', 'replication_factor': 1 }
    "#, &[]).await?;

    session
        .query_unpaged(
            r#"
      CREATE TABLE IF NOT EXISTS ks.messages (
        id uuid PRIMARY KEY,
        contents text
      )
    "#,
            &[],
        )
        .await?;

    Ok(())
}
