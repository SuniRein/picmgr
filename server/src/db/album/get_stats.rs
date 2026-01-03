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

#[instrument(skip(pool))]
pub async fn get_image_count_in_album(pool: &PgPool, album_id: i32) -> sqlx::Result<i64> {
    sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) as "count!"
        FROM image i
        JOIN image_album ia ON ia.image_id = i.id
        WHERE i.trashed_at IS NULL
          AND ia.album_id = $1
        "#,
        album_id
    )
    .fetch_one(pool)
    .await
    .inspect_err(|e| error!(error=?e, "fetch album image count failed"))
}
