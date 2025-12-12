use super::super::pagination::DbPagination;
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
pub async fn get_all_images(pool: &PgPool, pagination: DbPagination) -> sqlx::Result<Vec<Image>> {
    sqlx::query_as!(
        Image,
        "SELECT * FROM image ORDER BY id LIMIT $1 OFFSET $2",
        pagination.limit,
        pagination.offset,
    )
    .fetch_all(pool)
    .await
    .inspect_err(|e| error!(error=?e, "fetch image records failed"))
}

#[instrument(skip(pool))]
pub async fn get_image_by_id(pool: &PgPool, id: i32) -> sqlx::Result<Option<Image>> {
    sqlx::query_as!(Image, "SELECT * FROM image WHERE id = $1", id)
        .fetch_optional(pool)
        .await
        .inspect_err(|e| error!(error=?e, "fetch image record failed"))
}

#[instrument(skip(pool))]
pub async fn get_images_by_owner(
    pool: &PgPool,
    owner_id: i32,
    pagination: DbPagination,
) -> sqlx::Result<Vec<Image>> {
    sqlx::query_as!(
        Image,
        "SELECT * FROM image WHERE owner_id = $1 ORDER BY id LIMIT $2 OFFSET $3",
        owner_id,
        pagination.limit,
        pagination.offset,
    )
    .fetch_all(pool)
    .await
    .inspect_err(|e| error!(error=?e, "fetch image records failed"))
}
