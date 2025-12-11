use super::utils::parse_token;
use crate::api::error::AuthError;
use axum::{extract::FromRequestParts, http::request::Parts};
use std::fmt::{self, Debug, Formatter};

pub struct AdminClaims;

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
        Ok(AdminClaims)
    }
}

impl Debug for AdminClaims {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, r#"{{ role: "admin" }}"#)
    }
}
