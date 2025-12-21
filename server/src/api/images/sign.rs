use crate::auth::sign::{sign_data, verify_signature};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct SignedQuery {
    exp: i64,
    sig: String,
}

const SIGNED_QUERY_EXPIRY: Duration = Duration::minutes(5);

impl SignedQuery {
    pub fn generate(image_id: i32, now: DateTime<Utc>) -> Self {
        let exp = (now + SIGNED_QUERY_EXPIRY).timestamp();
        let data = format!("{image_id}|{exp}");
        let sig = sign_data(&data);
        Self { exp, sig }
    }

    pub fn verify(&self, image_id: i32, now: DateTime<Utc>) -> Result<(), VerifyError> {
        if self.exp < now.timestamp() {
            return Err(VerifyError::Expired);
        }

        let data = format!("{image_id}|{}", self.exp);
        match verify_signature(&data, &self.sig) {
            true => Ok(()),
            false => Err(VerifyError::InvalidSignature),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum VerifyError {
    #[error("Signature Expired")]
    Expired,
    #[error("Invalid Signature")]
    InvalidSignature,
}
