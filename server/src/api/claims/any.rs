use super::utils::parse_token;
use axum::{extract::FromRequestParts, http::request::Parts};
use std::fmt::{self, Debug, Formatter};

pub enum AnyClaims {
    Admin,
    User(i32),
    Guest,
}

impl<S> FromRequestParts<S> for AnyClaims
where
    S: Send + Sync,
{
    type Rejection = ();

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let claims = parse_token(parts, "access");
        Ok(match claims {
            Ok(c) if c.is_admin => Self::Admin,
            Ok(c) => Self::User(c.sub),
            Err(_) => Self::Guest,
        })
    }
}

impl Debug for AnyClaims {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Admin => write!(f, r#"{{ role: "admin" }}"#),
            Self::User(id) => write!(f, r#"{{ role: "user", user_id: {id} }}"#),
            Self::Guest => write!(f, r#"{{ role: "guest" }}"#),
        }
    }
}
