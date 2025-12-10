use super::utils::parse_token;
use crate::{api::error::AuthError, auth::jwt::Claims};
use axum::{extract::FromRequestParts, http::request::Parts};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaims(pub(super) Claims);

impl<S> FromRequestParts<S> for UserClaims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let claims = parse_token(parts, "access")?;
        if claims.is_admin {
            return Err(AuthError::InvalidToken);
        }
        Ok(UserClaims(claims))
    }
}

impl UserClaims {
    pub fn user_id(&self) -> i32 {
        self.0.sub
    }
}
