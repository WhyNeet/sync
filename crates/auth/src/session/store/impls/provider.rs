use std::str::FromStr;

use reqwest::{Client, IntoUrl, StatusCode, Url};
use uuid::Uuid;

use crate::session::{data::SessionData, store::SessionStore};

pub struct ProviderSessionStore {
    endpoint: Url,
    client: Client,
}

impl ProviderSessionStore {
    pub fn new(endpoint: impl IntoUrl) -> Result<Self, reqwest::Error> {
        Ok(Self {
            endpoint: endpoint.into_url()?,
            client: reqwest::Client::new(),
        })
    }
}

impl SessionStore for ProviderSessionStore {
    async fn load(
        &self,
        session_id: &uuid::Uuid,
    ) -> Result<crate::session::data::SessionData, Box<dyn std::error::Error>> {
        let uri = self.endpoint.join("/internal/validate")?;
        println!("validating session externally: {uri}");
        let response = self
            .client
            .post(uri)
            .body(session_id.to_string())
            .send()
            .await?;
        if response.status() != StatusCode::OK {
            return Err("invalid session.".into());
        }

        return Ok(SessionData {
            id: *session_id,
            user_id: Uuid::from_str(&response.text().await?)?,
        });
    }

    async fn save(&self, _: &uuid::Uuid, _: &uuid::Uuid) -> Result<(), Box<dyn std::error::Error>> {
        panic!("cannot save session using external provider.")
    }
}
