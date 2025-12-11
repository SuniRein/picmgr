use crate::api::error::{ApiError, ApiResult};
use crate::auth::password::hash_password;
use crate::db::user::{self, NewUserInput};
use axum::{Json, debug_handler, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use sqlx::PgPool;
use tracing::{info, instrument};
use validator::ValidateEmail;

#[derive(Deserialize)]
pub struct RegisterPayload {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[debug_handler]
#[instrument(skip(pool, payload), fields(username = %payload.username, email = %payload.email))]
pub async fn register(
    State(pool): State<PgPool>,
    Json(mut payload): Json<RegisterPayload>,
) -> ApiResult<impl IntoResponse> {
    payload.username = payload.username.trim().to_string();
    payload.email = payload.email.trim().to_string();
    payload.password = payload.password.trim().to_string();

    check_empty_fields(&payload)?;
    check_email_format(&payload.email)?;
    check_user_conflicts(&pool, &payload).await?;

    let password_hash = hash_password(&payload.password)?;
    let new_user = NewUserInput {
        username: &payload.username,
        email: &payload.email,
        password_hash: &password_hash,
    };
    user::create_user(&pool, &new_user).await?;
    info!("user registered successfully");

    Ok((StatusCode::CREATED, Json("User created successfully")))
}

fn check_empty_fields(payload: &RegisterPayload) -> ApiResult<()> {
    if payload.username.is_empty() {
        info!("empty username");
        return Err(ApiError::EmptyField("username".to_string()));
    }
    if payload.email.is_empty() {
        info!("empty email");
        return Err(ApiError::EmptyField("email".to_string()));
    }
    if payload.password.is_empty() {
        info!("empty password");
        return Err(ApiError::EmptyField("password".to_string()));
    }
    Ok(())
}

fn check_email_format(email: &str) -> ApiResult<()> {
    if !email.validate_email() {
        info!("invalid email format");
        return Err(ApiError::InvalidEmailFormat);
    }
    Ok(())
}

async fn check_user_conflicts(pool: &PgPool, payload: &RegisterPayload) -> ApiResult<()> {
    if user::get_user_by_username(pool, &payload.username)
        .await?
        .is_some()
    {
        info!("username conflict");
        return Err(ApiError::UsernameConflict);
    }
    if user::get_user_by_email(pool, &payload.email)
        .await?
        .is_some()
    {
        info!("email conflict");
        return Err(ApiError::EmailConflict);
    }
    Ok(())
}
