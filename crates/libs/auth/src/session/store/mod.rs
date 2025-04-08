pub mod impls;
pub mod integration;

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD as Base64Engine};
use cookie::{Cookie, SameSite, time::Duration};
use hmac::{Hmac, Mac};
use sha2::Sha256;

use std::{error::Error, str::FromStr};

use uuid::Uuid;

use super::data::SessionData;

type HmacSha256 = Hmac<Sha256>;

pub trait SessionStore {
    async fn save(&self, session_id: &Uuid, user_id: &Uuid) -> Result<(), Box<dyn Error>>;
    async fn load(&self, session_id: &Uuid) -> Result<SessionData, Box<dyn Error>>;
}

pub struct SessionManager<S: SessionStore> {
    store: S,
    signing_key: Vec<u8>, // Store the key bytes
    cookie_name: String,
    cookie_domain: Option<String>,
    cookie_path: String,
    cookie_secure: bool,
    cookie_http_only: bool,
    cookie_same_site: SameSite,
}

impl<S: SessionStore> SessionManager<S> {
    pub fn new(store: S, signing_key: &[u8]) -> Self {
        Self {
            store,
            signing_key: signing_key.to_vec(),
            cookie_name: "sid".to_string(),
            cookie_domain: None,
            cookie_path: "/".to_string(),
            cookie_secure: true,
            cookie_http_only: true,
            cookie_same_site: SameSite::None,
        }
    }

    fn sign(&self, session_id: &str) -> String {
        let mut mac = HmacSha256::new_from_slice(&self.signing_key).unwrap();
        mac.update(session_id.as_bytes());
        let signature = mac.finalize().into_bytes();
        Base64Engine.encode(signature)
    }

    fn verify(&self, session_id: &str, signature_b64: &str) -> bool {
        let Ok(signature) = Base64Engine.decode(signature_b64) else {
            return false;
        };

        let mut mac = HmacSha256::new_from_slice(&self.signing_key).unwrap();
        mac.update(session_id.as_bytes());
        mac.verify_slice(&signature).is_ok()
    }

    fn build_cookie<'a>(&self, session_id: &'a str, signature: &'a str) -> Cookie<'static> {
        let value = format!("{}.{}", session_id, signature);
        let mut builder = Cookie::build((self.cookie_name.clone(), value))
            .path(self.cookie_path.clone())
            .http_only(self.cookie_http_only)
            .same_site(self.cookie_same_site)
            .secure(self.cookie_secure);

        if let Some(domain) = &self.cookie_domain {
            builder = builder.domain(domain.clone());
        }

        let max_age = Duration::days(30);
        builder = builder.max_age(max_age);

        builder.build()
    }

    pub async fn create_session(
        &self,
        session_id: &Uuid,
        user_id: &Uuid,
    ) -> Result<Cookie<'static>, Box<dyn Error>> {
        let signature = self.sign(&session_id.to_string());
        self.store.save(&session_id, &user_id).await?;

        Ok(self.build_cookie(&session_id.to_string(), &signature))
    }

    pub async fn load_session(&self, sid: &str) -> Result<SessionData, Box<dyn Error>> {
        let (session_id, signature) = sid.split_once('.').ok_or("invalid session id.")?;
        if !self.verify(session_id, signature) {
            return Err("invalid signature.".into());
        }

        let session_id = Uuid::from_str(session_id)?;

        self.store.load(&session_id).await
    }

    pub async fn load_session_unverified(
        &self,
        session_id: &str,
    ) -> Result<SessionData, Box<dyn Error>> {
        let session_id = Uuid::from_str(session_id)?;

        self.store.load(&session_id).await
    }
}

#[derive(Debug, Clone)]
pub struct Session(pub SessionData);
