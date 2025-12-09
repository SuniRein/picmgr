use jsonwebtoken::{
    DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode,
    errors::ErrorKind as JwtErrorKind,
};
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::LazyLock;

#[derive(Debug)]
struct Key {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

static JWT_KEY: LazyLock<Key> = LazyLock::new(|| {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Key {
        encoding: EncodingKey::from_secret(secret.as_bytes()),
        decoding: DecodingKey::from_secret(secret.as_bytes()),
    }
});

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: usize,
    pub is_admin: bool,
    pub token_use: String,
}

pub fn decode_token(token: &str) -> Result<TokenData<Claims>, TokenError> {
    decode::<Claims>(token, &JWT_KEY.decoding, &Validation::default()).map_err(|err| {
        match *err.kind() {
            JwtErrorKind::ExpiredSignature => TokenError::ExpiredToken,
            _ => TokenError::InvalidToken,
        }
    })
}

pub fn encode_token(claims: &Claims) -> Option<String> {
    encode(&Header::default(), claims, &JWT_KEY.encoding).ok()
}

#[derive(Debug)]
pub enum TokenError {
    ExpiredToken,
    InvalidToken,
}
