use super::error::{ApiError, ApiResult};
use crate::auth::password::hash_password;
use crate::db::user::{self, NewUserInput};
use axum::{Json, debug_handler, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use sqlx::PgPool;
use validator::ValidateEmail;

#[derive(Deserialize)]
pub struct RegisterPayload {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[debug_handler]
pub async fn register(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterPayload>,
) -> ApiResult<impl IntoResponse> {
    check_email_format(&payload.email)?;
    check_user_conflicts(&pool, &payload).await?;

    let password_hash = hash_password(&payload.password)?;
    let new_user = NewUserInput {
        username: &payload.username,
        email: &payload.email,
        password_hash: &password_hash,
    };
    user::create_user(&pool, &new_user).await?;

    Ok((StatusCode::CREATED, Json("User created successfully")))
}

fn check_email_format(email: &str) -> ApiResult<()> {
    if !email.validate_email() {
        return Err(ApiError::InvalidEmailFormat);
    }
    Ok(())
}

async fn check_user_conflicts(pool: &PgPool, payload: &RegisterPayload) -> ApiResult<()> {
    if user::get_user_by_username(pool, &payload.username)
        .await?
        .is_some()
    {
        return Err(ApiError::UsernameConflict);
    }
    if user::get_user_by_email(pool, &payload.email)
        .await?
        .is_some()
    {
        return Err(ApiError::EmailConflict);
    }
    Ok(())
}
