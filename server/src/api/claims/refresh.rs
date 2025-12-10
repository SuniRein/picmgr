use super::utils::parse_token;
use crate::{api::error::AuthError, auth::jwt::Claims};
use axum::{extract::FromRequestParts, http::request::Parts};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshClaims(pub(super) Claims);

impl<S> FromRequestParts<S> for RefreshClaims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let claims = parse_token(parts, "refresh")?;
        Ok(RefreshClaims(claims))
    }
}
