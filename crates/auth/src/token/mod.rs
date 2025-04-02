use hmac::Hmac;
use sha2::Sha256;

pub mod access_token;
pub mod refresh_token;

pub trait Token {
    type Error;

    fn sign_with_key(self, key: &Hmac<Sha256>) -> Result<String, Self::Error>;
}
