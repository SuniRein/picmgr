use crate::image::ThumbnailSize;
use sqlx::{FromRow, PgPool};
use tracing::{error, instrument};

#[derive(Debug, FromRow)]
pub struct ImageStorageInfo {
    pub storage_key: String,
    pub mime_type: String,
    pub has_small_thumbnail: bool,
    pub has_medium_thumbnail: bool,
    pub has_large_thumbnail: bool,
}

impl ImageStorageInfo {
    pub fn has_thumbnail(&self, size: ThumbnailSize) -> bool {
        match size {
            ThumbnailSize::Small => self.has_small_thumbnail,
            ThumbnailSize::Medium => self.has_medium_thumbnail,
            ThumbnailSize::Large => self.has_large_thumbnail,
        }
    }
}

#[instrument(skip(pool))]
pub async fn get_image_storage_info(
    pool: &PgPool,
    id: i32,
) -> sqlx::Result<Option<ImageStorageInfo>> {
    sqlx::query_as!(
        ImageStorageInfo,
        "SELECT
          s.storage_key, s.mime_type,
          s.has_small_thumbnail, s.has_medium_thumbnail, s.has_large_thumbnail
         FROM image i
         JOIN image_storage s ON i.storage_id = s.id
         WHERE i.id = $1",
        id
    )
    .fetch_optional(pool)
    .await
    .inspect_err(|e| error!(error=?e, "fetch image storage info failed"))
}
