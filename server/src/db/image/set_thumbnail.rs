use crate::image::ThumbnailSize;
use sqlx::PgPool;
use tracing::{error, instrument};

#[instrument(skip(pool))]
pub async fn set_thumbnail_exists(pool: &PgPool, id: i32, size: ThumbnailSize) -> sqlx::Result<()> {
    match size {
        ThumbnailSize::Small => sqlx::query!(
            "UPDATE image_storage s
             SET has_small_thumbnail = TRUE
             FROM image i
             WHERE i.storage_id = s.id AND i.id = $1",
            id
        ),
        ThumbnailSize::Medium => sqlx::query!(
            "UPDATE image_storage s
             SET has_medium_thumbnail = TRUE
             FROM image i
             WHERE i.storage_id = s.id AND i.id = $1",
            id
        ),
        ThumbnailSize::Large => sqlx::query!(
            "UPDATE image_storage s
             SET has_large_thumbnail = TRUE
             FROM image i
             WHERE i.storage_id = s.id AND i.id = $1",
            id
        ),
    }
    .execute(pool)
    .await
    .map(|_| ())
    .inspect_err(|e| error!(error=?e, "set thumbnail exists failed"))
}
