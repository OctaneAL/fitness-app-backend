use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

extern crate argon2;

use argon2::Config;

const SECRET: &[u8] = b"yWyT3H-BRJAwkbOVHYKbVR6wwLYoQrFj00lXp1PfqGnM";

pub fn create_jwt(username: &str) -> String {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() + 60*60; // Token expires in 1 hour

    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET)).expect("JWT encoding should work")
}

pub fn decode_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(token, &DecodingKey::from_secret(SECRET), &Validation::new(Algorithm::HS256)).map(|data| data.claims)
}

pub fn generate_salt() -> [u8; 16] {
    use rand::{rngs::OsRng, RngCore};

    let mut salt = [0u8; 16];
    let mut rng = OsRng::default();
    rng.fill_bytes(&mut salt);
    salt
}

pub fn verify_password(hashed_password: &str, password: &str) -> bool {
    argon2::verify_encoded(hashed_password, password.as_bytes()).unwrap_or(false)
}

pub fn hash_password(password: &str) -> String {
    let salt = generate_salt();
    let config = Config::default();

    argon2::hash_encoded(password.as_bytes(), &salt, &config).unwrap()
}