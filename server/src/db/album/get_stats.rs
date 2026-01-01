use sqlx::PgPool;
use tracing::{error, instrument};

#[instrument(skip(pool))]
pub async fn get_user_album_count(pool: &PgPool, user_id: i32) -> sqlx::Result<i64> {
    sqlx::query_scalar!(
        r#"SELECT COUNT(*) as "count!" FROM album WHERE owner_id = $1"#,
        user_id
    )
    .fetch_one(pool)
    .await
    .inspect_err(|e| error!(error=?e, "fetch user album count failed"))
}
