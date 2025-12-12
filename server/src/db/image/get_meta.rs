use super::super::pagination::DbPagination;
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{FromRow, PgPool};
use tracing::{error, instrument};

#[derive(Debug, FromRow, Serialize, utoipa::ToSchema)]
pub struct ImageMeta {
    id: i32,
    owner_id: Option<i32>,
    category_id: Option<i32>,

    size_bytes: i64,
    width: i32,
    height: i32,
    mime_type: String,
    exif: Option<serde_json::Value>,

    is_public: bool,

    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[instrument(skip(pool))]
pub async fn get_all_image_metas(
    pool: &PgPool,
    pagination: DbPagination,
) -> sqlx::Result<Vec<ImageMeta>> {
    sqlx::query_as!(
        ImageMeta,
        "SELECT
          id, owner_id, category_id, size_bytes, width, height, mime_type, exif,
          is_public, created_at, updated_at
         FROM image
         ORDER BY id
         LIMIT $1 OFFSET $2",
        pagination.limit,
        pagination.offset,
    )
    .fetch_all(pool)
    .await
    .inspect_err(|e| error!(error=?e, "fetch image records failed"))
}

#[instrument(skip(pool))]
pub async fn get_image_meta_by_id(pool: &PgPool, id: i32) -> sqlx::Result<Option<ImageMeta>> {
    sqlx::query_as!(
        ImageMeta,
        "SELECT
          id, owner_id, category_id, size_bytes, width, height, mime_type, exif,
          is_public, created_at, updated_at
         FROM image
         WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await
    .inspect_err(|e| error!(error=?e, "fetch image record failed"))
}

#[instrument(skip(pool))]
pub async fn get_image_metas_by_owner(
    pool: &PgPool,
    owner_id: i32,
    pagination: DbPagination,
) -> sqlx::Result<Vec<ImageMeta>> {
    sqlx::query_as!(
        ImageMeta,
        "SELECT
          id, owner_id, category_id, size_bytes, width, height, mime_type, exif,
          is_public, created_at, updated_at
         FROM image
         WHERE owner_id = $1
         ORDER BY id
         LIMIT $2 OFFSET $3",
        owner_id,
        pagination.limit,
        pagination.offset,
    )
    .fetch_all(pool)
    .await
    .inspect_err(|e| error!(error=?e, "fetch image records failed"))
}
