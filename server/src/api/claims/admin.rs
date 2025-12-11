use super::utils::parse_token;
use crate::{api::error::AuthError, auth::jwt::Claims};
use axum::{extract::FromRequestParts, http::request::Parts};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug, Formatter};

#[derive(Serialize, Deserialize)]
pub struct AdminClaims(pub(super) Claims);

impl<S> FromRequestParts<S> for AdminClaims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let claims = parse_token(parts, "access")?;
        if !claims.is_admin {
            return Err(AuthError::AdminRequired);
        }
        Ok(AdminClaims(claims))
    }
}

impl Debug for AdminClaims {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, r#"{{ role: "admin" }}"#)
    }
}
