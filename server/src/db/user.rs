use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::PgPool;

#[derive(Debug, sqlx::Type, Serialize)]
#[sqlx(type_name = "user_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum UserStatus {
    Active,
    Banned,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub avatar_url: Option<String>,
    pub status: UserStatus,
    pub created_at: NaiveDateTime,
}

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
}

pub async fn get_all_users(pool: &PgPool) -> sqlx::Result<Vec<User>> {
    sqlx::query_as!(
        User,
        r#"
         SELECT id, username, email, password_hash, avatar_url, status as "status: _", created_at
         FROM app_user
        "#
    )
    .fetch_all(pool)
    .await
}

#[derive(Debug)]
pub struct NewUserInput<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password_hash: &'a str,
}

pub async fn create_user(pool: &PgPool, new_user: &NewUserInput<'_>) -> sqlx::Result<User> {
    sqlx::query_as!(
        User,
        r#"
         INSERT INTO app_user (username, email, password_hash)
         VALUES ($1, $2, $3)
         RETURNING id, username, email, password_hash, avatar_url, status as "status: _", created_at
        "#,
        new_user.username,
        new_user.email,
        new_user.password_hash
    )
    .fetch_one(pool)
    .await
}
