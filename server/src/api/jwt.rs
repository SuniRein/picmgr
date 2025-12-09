use super::error::AuthError;
use crate::auth::jwt::{Claims, TokenError, decode_token, encode_token};
use axum::{extract::FromRequestParts, http::request::Parts};
use chrono::Duration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaims(Claims);

impl<S> FromRequestParts<S> for UserClaims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let claims = parse_access_token(parts)?;
        Ok(UserClaims(claims))
    }
}

impl UserClaims {
    pub fn user_id(&self) -> i32 {
        self.0.sub
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminClaims(Claims);

impl<S> FromRequestParts<S> for AdminClaims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let claims = parse_access_token(parts)?;
        if !claims.is_admin {
            return Err(AuthError::AdminRequired);
        }
        Ok(AdminClaims(claims))
    }
}

fn parse_access_token(parts: &mut Parts) -> Result<Claims, AuthError> {
    let auth_header = parts
        .headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or(AuthError::InvalidToken)?;

    let token_data = decode_token(auth_header).map_err(|e| match e {
        TokenError::ExpiredToken => AuthError::ExpiredToken,
        TokenError::InvalidToken => AuthError::InvalidToken,
    })?;

    if token_data.claims.token_use != "access" {
        return Err(AuthError::InvalidToken);
    }

    Ok(token_data.claims)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    access_token: String,
    refresh_token: String,
}

const ACCESS_TOKEN_EXPIRY: Duration = Duration::minutes(15);
const REFRESH_TOKEN_EXPIRY: Duration = Duration::days(7);

impl Token {
    pub fn generate_user_token(user_id: i32) -> Option<Self> {
        let access_claims = Claims {
            sub: user_id,
            exp: (chrono::Utc::now() + ACCESS_TOKEN_EXPIRY).timestamp() as usize,
            is_admin: false,
            token_use: "access".to_string(),
        };
        let refresh_claims = Claims {
            sub: user_id,
            exp: (chrono::Utc::now() + REFRESH_TOKEN_EXPIRY).timestamp() as usize,
            is_admin: false,
            token_use: "refresh".to_string(),
        };

        let access_token = encode_token(&access_claims)?;
        let refresh_token = encode_token(&refresh_claims)?;

        Some(Token {
            access_token,
            refresh_token,
        })
    }

    pub fn generate_admin_token() -> Option<Self> {
        let access_claims = Claims {
            sub: 0,
            exp: (chrono::Utc::now() + ACCESS_TOKEN_EXPIRY).timestamp() as usize,
            is_admin: true,
            token_use: "access".to_string(),
        };
        let refresh_claims = Claims {
            sub: 0,
            exp: (chrono::Utc::now() + REFRESH_TOKEN_EXPIRY).timestamp() as usize,
            is_admin: true,
            token_use: "refresh".to_string(),
        };

        let access_token = encode_token(&access_claims)?;
        let refresh_token = encode_token(&refresh_claims)?;

        Some(Token {
            access_token,
            refresh_token,
        })
    }
}
