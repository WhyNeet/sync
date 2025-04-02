use std::{collections::BTreeMap, error::Error};

use chrono::{TimeDelta, Utc};
use jwt::SignWithKey;

use crate::util;

use super::Token;

pub struct AccessToken {
    pub token_id: String,
    pub user_id: String,
    pub lifetime: u64,
}

impl AccessToken {
    pub fn new(token_id: String, user_id: String, lifetime: u64) -> Self {
        Self {
            token_id,
            user_id,
            lifetime,
        }
    }
}

impl Token for AccessToken {
    type Error = Box<dyn Error>;
    fn sign_with_key(self, key: &hmac::Hmac<sha2::Sha256>) -> Result<String, Self::Error> {
        let mut claims = BTreeMap::new();
        claims.insert("jti", self.token_id);
        claims.insert("subject", self.user_id);
        let (iat, exp) = util::calculate_token_params(self.lifetime);
        claims.insert("iat", iat.timestamp().to_string());
        claims.insert("exp", exp.timestamp().to_string());
        let token_str = claims.sign_with_key(key)?;

        Ok(token_str)
    }
}
