use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use hmac_sha256::HMAC;
use std::env;
use std::sync::LazyLock;

static SIGN_SECRET: LazyLock<String> =
    LazyLock::new(|| env::var("SIGN_SECRET").expect("SIGN_SECRET must be set"));

pub fn sign_data(data: &str) -> String {
    let signature = HMAC::mac(data.as_bytes(), SIGN_SECRET.as_bytes());
    URL_SAFE_NO_PAD.encode(signature)
}

pub fn verify_signature(data: &str, signature: &str) -> bool {
    let expected_signature = HMAC::mac(data.as_bytes(), SIGN_SECRET.as_bytes());
    match URL_SAFE_NO_PAD.decode(signature) {
        Ok(decoded_signature) => expected_signature.as_slice() == decoded_signature.as_slice(),
        Err(_) => false,
    }
}
