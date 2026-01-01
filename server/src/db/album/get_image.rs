use super::super::{image::ImageMeta, pagination::DbPagination};
use sqlx::PgPool;
use tracing::{error, instrument};

#[instrument(skip(pool))]
pub async fn get_images_in_album(
    pool: &PgPool,
    album_id: i32,
    pagination: DbPagination,
) -> sqlx::Result<Vec<ImageMeta>> {
    sqlx::query_as!(
        ImageMeta,
        r#"
        WITH img AS (
          SELECT
            i.id, i.owner_id,
            s.size_bytes, s.width, s.height, s.mime_type, s.exif,
            i.is_public, i.created_at, i.updated_at,
            ia.position
          FROM image_album ia
          JOIN image i ON ia.image_id = i.id
          JOIN image_storage s ON i.storage_id = s.id
          WHERE ia.album_id = $1
          ORDER BY ia.position DESC
          LIMIT $2 OFFSET $3
        )
        SELECT
          img.id, img.owner_id,
          img.size_bytes, img.width, img.height, img.mime_type, img.exif,
          img.is_public, img.created_at, img.updated_at,
          COALESCE(tl.tags, '{}'::text[]) AS "tags!"
        FROM img
        LEFT JOIN LATERAL (
          SELECT array_agg(t.name ORDER BY t.name) AS tags
          FROM image_tag it
          JOIN tag t ON it.tag_id = t.id
          WHERE it.image_id = img.id
        ) tl on TRUE
        ORDER BY img.position DESC
        "#,
        album_id,
        pagination.limit,
        pagination.offset,
    )
    .fetch_all(pool)
    .await
    .inspect_err(|e| error!(error=?e, "fetch images in album failed"))
}
