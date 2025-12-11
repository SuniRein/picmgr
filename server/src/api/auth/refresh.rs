use super::super::{
    claims::{AccessToken, RefreshClaims},
    doc::AUTH_TAG,
    error::ApiResult,
};
use axum::{Json, debug_handler, response::IntoResponse};
use serde::Serialize;
use tracing::{info, instrument};

/// Refresh access token
///
/// Generates a new access token using a valid refresh token.
#[utoipa::path(
    post,
    tag = AUTH_TAG,
    path = "/auth/refresh",
    security(("refreshToken" = [])),
    responses(
        (status = OK, description = "success response", body = TokenResponse),
        (status = UNAUTHORIZED, description = "invalid or expired refresh token"),
    ),
)]
#[debug_handler]
#[instrument]
pub async fn refresh_token(claims: RefreshClaims) -> ApiResult<impl IntoResponse> {
    let access_token = claims.refresh();
    info!("access token refreshed");
    Ok(Json(TokenResponse { access_token }))
}

#[derive(Serialize, utoipa::ToSchema)]
struct TokenResponse {
    access_token: AccessToken,
}
