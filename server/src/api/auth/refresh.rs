use crate::api::{
    claims::{RefreshClaims, Token},
    error::ApiResult,
};
use axum::{Json, debug_handler, response::IntoResponse};
use serde_json::json;
use tracing::{info, instrument};

#[debug_handler]
#[instrument]
pub async fn refresh_token(claims: RefreshClaims) -> ApiResult<impl IntoResponse> {
    let access_token = Token::refresh(&claims);
    info!("access token refreshed");
    Ok(Json(json!({ "access_token": access_token })))
}
