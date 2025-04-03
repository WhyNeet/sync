use std::{error::Error, sync::Arc};

use scylla::{client::session::Session, statement::prepared::PreparedStatement};
use uuid::Uuid;

use crate::session::{data::SessionData, store::SessionStore};

pub struct ScyllaSessionStore {
    db_session: Arc<Session>,
    save_stmt: Arc<PreparedStatement>,
    load_stmt: Arc<PreparedStatement>,
}

impl ScyllaSessionStore {
    pub async fn new(
        db_session: Arc<Session>,
        keyspace: String,
        table: String,
    ) -> Result<Self, Box<dyn Error>> {
        let save_cql = format!(
            "INSERT INTO {}.{} (id, user_id) VALUES (?, ?)",
            keyspace, table
        );
        let load_cql = format!("SELECT * FROM {}.{} WHERE id = ? LIMIT 1", keyspace, table);

        let save_stmt = Arc::new(db_session.prepare(save_cql).await?);
        let load_stmt = Arc::new(db_session.prepare(load_cql).await?);

        Ok(Self {
            db_session,
            load_stmt,
            save_stmt,
        })
    }
}

impl SessionStore for ScyllaSessionStore {
    async fn load(
        &self,
        session_id: &Uuid,
    ) -> Result<crate::session::data::SessionData, Box<dyn Error>> {
        let result = self
            .db_session
            .execute_unpaged(&self.load_stmt, (session_id,))
            .await?;
        let (id, user_id) = result.into_rows_result()?.first_row::<(Uuid, Uuid)>()?;

        Ok(SessionData { id, user_id })
    }

    async fn save(&self, session_id: &Uuid, user_id: &uuid::Uuid) -> Result<(), Box<dyn Error>> {
        self.db_session
            .execute_unpaged(&self.save_stmt, (session_id, user_id))
            .await?;
        Ok(())
    }
}
