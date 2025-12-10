use chrono::NaiveDateTime;
use sqlx::{FromRow, PgPool};
use tracing::{error, instrument};

#[derive(Debug, FromRow)]
pub struct Image {
    pub id: i32,
    pub owner_id: Option<i32>,
    pub category_id: Option<i32>,

    pub storage_key: String,
    pub size_bytes: i64,
    pub width: i32,
    pub height: i32,
    pub mime_type: String,
    pub exif: Option<serde_json::Value>,

    pub small_thumbnail_key: Option<String>,
    pub medium_thumbnail_key: Option<String>,
    pub large_thumbnail_key: Option<String>,

    pub is_public: bool,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[instrument(skip(pool))]
pub async fn get_all_images(pool: &PgPool) -> sqlx::Result<Vec<Image>> {
    sqlx::query_as!(Image, "SELECT * FROM image")
        .fetch_all(pool)
        .await
        .inspect_err(|e| error!("Failed to fetch images from database: {e}"))
}

#[instrument(skip(pool))]
pub async fn get_image_by_id(pool: &PgPool, id: i32) -> sqlx::Result<Option<Image>> {
    sqlx::query_as!(Image, "SELECT * FROM image WHERE id = $1", id)
        .fetch_optional(pool)
        .await
        .inspect_err(|e| error!("Failed to fetch image: {e}"))
}

#[instrument(skip(pool))]
pub async fn get_images_by_owner(pool: &PgPool, owner_id: i32) -> sqlx::Result<Vec<Image>> {
    sqlx::query_as!(Image, "SELECT * FROM image WHERE owner_id = $1", owner_id)
        .fetch_all(pool)
        .await
        .inspect_err(|e| error!("Failed to fetch images: {e}"))
}

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

    pub small_thumbnail_key: Option<&'a str>,
    pub medium_thumbnail_key: Option<&'a str>,
    pub large_thumbnail_key: Option<&'a str>,

    pub is_public: bool,
}

#[instrument(skip(pool, info), fields(owner_id = info.owner_id, category_id = info.category_id, storage_key = info.storage_key))]
pub async fn create_image(pool: &PgPool, info: NewImageInput<'_>) -> sqlx::Result<Image> {
    sqlx::query_as!(
        Image,
        r#"
        INSERT INTO image (
            owner_id, category_id,
            storage_key, size_bytes, width, height, mime_type, exif,
            small_thumbnail_key, medium_thumbnail_key, large_thumbnail_key,
            is_public
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        RETURNING *
        "#,
        info.owner_id,
        info.category_id,
        info.storage_key,
        info.size_bytes,
        info.width,
        info.height,
        info.mime_type,
        info.exif,
        info.small_thumbnail_key,
        info.medium_thumbnail_key,
        info.large_thumbnail_key,
        info.is_public
    )
    .fetch_one(pool)
    .await
    .inspect_err(|e| error!("Failed to create image record in database: {e}"))
}
