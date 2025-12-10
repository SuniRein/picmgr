use super::utils::parse_token;
use crate::api::error::AuthError;
use axum::{extract::FromRequestParts, http::request::Parts};
use std::fmt::{self, Debug, Formatter};

pub enum AccessClaims {
    User(i32),
    Admin,
}

impl<S> FromRequestParts<S> for AccessClaims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let claims = parse_token(parts, "access")?;
        match claims.is_admin {
            true => Ok(AccessClaims::Admin),
            false => Ok(AccessClaims::User(claims.sub)),
        }
    }
}

impl Debug for AccessClaims {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AccessClaims::User(id) => write!(f, r#"{{ role: "user", user_id: {id} }}"#),
            AccessClaims::Admin => write!(f, r#"{{ role: "admin" }}"#),
        }
    }
}
