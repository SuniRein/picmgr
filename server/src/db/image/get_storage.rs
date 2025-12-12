use sqlx::{FromRow, PgPool};
use tracing::{error, instrument};

#[derive(Debug, FromRow)]
pub struct ImageStorageInfo {
    pub storage_key: String,
    pub mime_type: String,
}

#[instrument(skip(pool))]
pub async fn get_image_storage_info(
    pool: &PgPool,
    id: i32,
) -> sqlx::Result<Option<ImageStorageInfo>> {
    sqlx::query_as!(
        ImageStorageInfo,
        "SELECT storage_key, mime_type FROM image WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await
    .inspect_err(|e| error!(error=?e, "fetch image storage info failed"))
}
