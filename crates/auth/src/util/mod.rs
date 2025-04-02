use chrono::{DateTime, TimeDelta, Utc};
use hmac::{Hmac, Mac, digest::InvalidLength};
use sha2::Sha256;

pub fn key_from_slice(s: &[u8]) -> Result<Hmac<Sha256>, InvalidLength> {
    Hmac::new_from_slice(s)
}

pub fn calculate_token_params(lifetime: u64) -> (DateTime<Utc>, DateTime<Utc>) {
    let iat = Utc::now();
    let exp = iat
        .checked_add_signed(TimeDelta::seconds(lifetime as i64))
        .unwrap();

    (iat, exp)
}
