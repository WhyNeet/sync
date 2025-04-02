use std::error::Error;

use argon2::{
    Argon2, Params, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};

pub trait Hasher {
    fn hash_str(&self, bytes: &[u8]) -> Result<String, Box<dyn Error>>;
    fn check_str(&self, bytes: &[u8], hash: &str) -> Result<bool, Box<dyn Error>>;
}

pub struct Argon2idHasher<'a> {
    instance: Argon2<'a>,
}

impl<'a> Argon2idHasher<'a> {
    pub fn new() -> Self {
        let argon2 = Argon2::new(
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            Params::new(7168, 5, 1, None).unwrap(),
        );

        Self { instance: argon2 }
    }
}

impl<'a> Hasher for Argon2idHasher<'a> {
    fn hash_str(&self, bytes: &[u8]) -> Result<String, Box<dyn Error>> {
        let salt = SaltString::generate(&mut OsRng);

        Ok(self.instance.hash_password(bytes, &salt)?.to_string())
    }

    fn check_str(&self, bytes: &[u8], hash: &str) -> Result<bool, Box<dyn Error>> {
        let hash = PasswordHash::new(hash)?;
        Ok(self.instance.verify_password(bytes, &hash).is_ok())
    }
}

pub fn initialize_hasher() -> impl Hasher {
    Argon2idHasher::new()
}
