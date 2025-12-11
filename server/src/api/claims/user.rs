use super::utils::parse_token;
use crate::api::error::AuthError;
use axum::{extract::FromRequestParts, http::request::Parts};
use std::fmt::{self, Debug, Formatter};

pub struct UserClaims(i32);

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
        Ok(UserClaims(claims.sub))
    }
}

impl UserClaims {
    pub fn user_id(&self) -> i32 {
        self.0
    }
}

impl Debug for UserClaims {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, r#"{{ role: "user", user_id: {} }}"#, self.user_id())
    }
}
