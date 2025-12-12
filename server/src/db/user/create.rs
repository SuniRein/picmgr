use super::get::User;
use sqlx::PgPool;
use tracing::{error, instrument};

#[derive(Debug)]
pub struct NewUserInput<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password_hash: &'a str,
}

#[instrument(skip(pool, new_user), fields(username=new_user.username, email=new_user.email))]
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
    .inspect_err(|e| error!(error=?e, "create user failed"))
}
