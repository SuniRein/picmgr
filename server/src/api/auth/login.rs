use crate::api::{
    error::{ApiError, ApiResult},
    jwt::Token,
};
use crate::auth::{admin::verify_admin, password::verify_password};
use crate::db::user;
use axum::{Json, debug_handler, extract::State, response::IntoResponse};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    username: String,
    password: String,
}

#[debug_handler]
pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginPayload>,
) -> ApiResult<impl IntoResponse> {
    let user = user::get_user_by_username(&pool, &payload.username)
        .await?
        .ok_or(ApiError::WrongCredentials)?;

    if !verify_password(&payload.password, &user.password_hash)? {
        return Err(ApiError::WrongCredentials);
    }

    Ok(Json(Token::generate_user_token(user.id)))
}

#[debug_handler]
pub async fn login_as_admin(Json(payload): Json<LoginPayload>) -> ApiResult<impl IntoResponse> {
    if !verify_admin(&payload.username, &payload.password) {
        return Err(ApiError::WrongCredentials);
    }
    Ok(Json(Token::generate_admin_token()))
}
