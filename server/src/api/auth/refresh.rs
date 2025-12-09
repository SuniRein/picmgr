use crate::api::{
    error::ApiResult,
    jwt::{RefreshClaims, Token},
};
use axum::{Json, debug_handler, response::IntoResponse};
use serde_json::json;

#[debug_handler]
pub async fn refresh_token(claims: RefreshClaims) -> ApiResult<impl IntoResponse> {
    Ok(Json(json!({
        "access_token": Token::refresh(&claims)
    })))
}
