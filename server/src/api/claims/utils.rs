use crate::{
    api::error::AuthError,
    auth::jwt::{Claims, TokenError, decode_token},
};
use axum::http::request::Parts;

pub(super) fn parse_token(parts: &mut Parts, expected_use: &str) -> Result<Claims, AuthError> {
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

    if token_data.claims.token_use != expected_use {
        return Err(AuthError::InvalidToken);
    }

    Ok(token_data.claims)
}
