use sqlx::PgPool;
use tracing::{error, instrument};

#[derive(Debug)]
pub struct NewImageInput<'a> {
    pub owner_id: Option<i32>,
    pub category_id: Option<i32>,

    pub storage_key: &'a str,
    pub size_bytes: i64,
    pub width: i32,
    pub height: i32,
    pub mime_type: &'a str,
    pub exif: Option<&'a serde_json::Value>,

    pub has_small_thumbnail: bool,
    pub has_medium_thumbnail: bool,
    pub has_large_thumbnail: bool,

    pub is_public: bool,
}

#[instrument(skip(pool, info), fields(owner_id = info.owner_id, category_id = info.category_id, storage_key = info.storage_key))]
pub async fn create_image(pool: &PgPool, info: NewImageInput<'_>) -> sqlx::Result<i32> {
    sqlx::query_scalar!(
        r#"
        INSERT INTO image (
            owner_id, category_id,
            storage_key, size_bytes, width, height, mime_type, exif,
            has_small_thumbnail, has_medium_thumbnail, has_large_thumbnail,
            is_public
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        RETURNING id
        "#,
        info.owner_id,
        info.category_id,
        info.storage_key,
        info.size_bytes,
        info.width,
        info.height,
        info.mime_type,
        info.exif,
        info.has_small_thumbnail,
        info.has_medium_thumbnail,
        info.has_large_thumbnail,
        info.is_public
    )
    .fetch_one(pool)
    .await
    .inspect_err(|e| error!(error=?e, "create image record failed"))
}
