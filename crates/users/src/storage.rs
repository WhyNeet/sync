use std::env;

use scylla::client::{session::Session, session_builder::SessionBuilder};

pub async fn create_session() -> Result<Session, Box<dyn std::error::Error>> {
    let uri = env::var("SCYLLA_URI")?;
    tracing::info!("connecting to scylla at: \"{uri}\"");
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
      CREATE TABLE IF NOT EXISTS ks.users (
        id uuid,
        username text,
        email text,
        password text,
        display_name text,
        avatar_id uuid,
        PRIMARY KEY (id)
      )
    "#,
            &[],
        )
        .await?;

    session
        .query_unpaged(
            r#"
      CREATE TABLE IF NOT EXISTS ks.users_by_username (
        id uuid,
        username text,
        PRIMARY KEY (username)
      )
    "#,
            &[],
        )
        .await?;

    Ok(())
}
