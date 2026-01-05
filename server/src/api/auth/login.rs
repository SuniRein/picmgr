use super::super::{
    claims::Token,
    doc::AUTH_TAG,
    error::{ApiError, ApiResult},
};
use crate::{
    auth::{admin::verify_admin, password::verify_password},
    db::user,
};
use axum::{Json, debug_handler, extract::State, response::IntoResponse};
use serde::Deserialize;
use sqlx::PgPool;
use tracing::{info, instrument};

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct LoginPayload {
    identifier: String,
    password: String,
}

/// User login endpoint
///
/// Authenticates a user and returns JWT tokens upon successful login.
#[utoipa::path(
    post,
    tag = AUTH_TAG,
    path = "/auth/login",
    responses(
        (status = OK, description = "success response", body = Token),
        (status = UNAUTHORIZED, description = "wrong credentials"),
    ),
)]
#[debug_handler]
#[instrument(skip(pool, payload), fields(identifier = %payload.identifier))]
pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginPayload>,
) -> ApiResult<impl IntoResponse> {
    let user = user::get_user_by_identifier(&pool, &payload.identifier)
        .await?
        .ok_or_else(|| {
            info!("user not found");
            ApiError::WrongCredentials
        })?;

    if !verify_password(&payload.password, &user.password_hash)? {
        info!("password mismatch");
        return Err(ApiError::WrongCredentials);
    }

    let token = Token::generate_user_token(user.id);
    info!("user logged in");
    Ok(Json(token))
}

/// Admin login endpoint
///
/// Authenticates an admin and returns JWT tokens upon successful login.
#[utoipa::path(
    post,
    tag = AUTH_TAG,
    path = "/auth/login/admin",
    responses(
        (status = OK, description = "success response", body = Token),
        (status = UNAUTHORIZED, description = "wrong credentials"),
    ),
)]
#[debug_handler]
#[instrument(skip(payload), fields(%payload.identifier))]
pub async fn login_as_admin(Json(payload): Json<LoginPayload>) -> ApiResult<impl IntoResponse> {
    if !verify_admin(&payload.identifier, &payload.password) {
        info!("wrong admin credentials");
        return Err(ApiError::WrongCredentials);
    }

    let token = Token::generate_admin_token();
    info!("admin logged in");
    Ok(Json(token))
}
