use super::super::pagination::DbPagination;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use tracing::{error, instrument};

#[derive(Debug, sqlx::Type, Serialize, utoipa::ToSchema)]
#[sqlx(type_name = "user_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum UserStatus {
    Active,
    Banned,
}

#[derive(Debug, sqlx::FromRow, Serialize, utoipa::ToSchema)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub avatar_url: Option<String>,
    pub status: UserStatus,
    pub created_at: DateTime<Utc>,
}

#[instrument(skip(pool))]
pub async fn get_user_by_id(pool: &PgPool, user_id: i32) -> sqlx::Result<Option<User>> {
    sqlx::query_as!(
        User,
        r#"
         SELECT id, username, email, password_hash, avatar_url, status as "status: _", created_at
         FROM app_user
         WHERE id = $1
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await
    .inspect_err(|e| error!(error=?e, "fetch user record failed"))
}

#[instrument(skip(pool))]
pub async fn get_user_by_username(pool: &PgPool, username: &str) -> sqlx::Result<Option<User>> {
    sqlx::query_as!(
        User,
        r#"
         SELECT id, username, email, password_hash, avatar_url, status as "status: _", created_at
         FROM app_user
         WHERE username = $1
        "#,
        username
    )
    .fetch_optional(pool)
    .await
    .inspect_err(|e| error!(error=?e, "fetch user record failed"))
}

#[instrument(skip(pool))]
pub async fn get_user_by_email(pool: &PgPool, email: &str) -> sqlx::Result<Option<User>> {
    sqlx::query_as!(
        User,
        r#"
         SELECT id, username, email, password_hash, avatar_url, status as "status: _", created_at
         FROM app_user
         WHERE email = $1
        "#,
        email
    )
    .fetch_optional(pool)
    .await
    .inspect_err(|e| error!(error=?e, "fetch user record failed"))
}

#[instrument(skip(pool))]
pub async fn get_user_by_identifier(pool: &PgPool, identifier: &str) -> sqlx::Result<Option<User>> {
    sqlx::query_as!(
        User,
        r#"
         SELECT id, username, email, password_hash, avatar_url, status as "status: _", created_at
         FROM app_user
         WHERE username = $1 OR email = $1
        "#,
        identifier
    )
    .fetch_optional(pool)
    .await
    .inspect_err(|e| error!(error=?e, "fetch user record failed"))
}

#[instrument(skip(pool))]
pub async fn get_all_users(pool: &PgPool, pagination: DbPagination) -> sqlx::Result<Vec<User>> {
    sqlx::query_as!(
        User,
        r#"
         SELECT id, username, email, password_hash, avatar_url, status as "status: _", created_at
         FROM app_user
         ORDER BY id
         LIMIT $1 OFFSET $2
        "#,
        pagination.limit,
        pagination.offset
    )
    .fetch_all(pool)
    .await
    .inspect_err(|e| error!(error=?e, "fetch user records failed"))
}
