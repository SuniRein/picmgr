use super::super::pagination::DbPagination;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{FromRow, PgPool};
use tracing::{error, instrument};

#[derive(Debug, FromRow, Serialize, utoipa::ToSchema)]
pub struct ImageMeta {
    pub id: i32,
    pub owner_id: Option<i32>,

    pub size_bytes: i64,
    pub width: i32,
    pub height: i32,
    pub mime_type: String,
    pub exif: Option<serde_json::Value>,

    pub is_public: bool,

    pub tags: Vec<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[instrument(skip(pool))]
pub async fn get_all_image_metas(
    pool: &PgPool,
    pagination: DbPagination,
) -> sqlx::Result<Vec<ImageMeta>> {
    sqlx::query_as!(
        ImageMeta,
        r#"
         WITH img AS (
           SELECT
             i.id, i.owner_id,
             s.size_bytes, s.width, s.height, s.mime_type, s.exif,
             i.is_public, i.created_at, i.updated_at
           FROM image i
           JOIN image_storage s ON i.storage_id = s.id
           ORDER BY id
           LIMIT $1 OFFSET $2
         )
         SELECT img.*, COALESCE(tl.tags, '{}'::text[]) AS "tags!"
         FROM img
         LEFT JOIN LATERAL (
           SELECT array_agg(t.name ORDER BY t.name) AS tags
           FROM image_tag it
           JOIN tag t ON it.tag_id = t.id
           WHERE it.image_id = img.id
         ) tl on TRUE
         ORDER BY img.id
        "#,
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
        r#"
        SELECT
          i.id, i.owner_id,
          s.size_bytes, s.width, s.height, s.mime_type, s.exif,
          i.is_public, i.created_at, i.updated_at,
          COALESCE(tl.tags, '{}'::text[]) AS "tags!"
         FROM image i
         JOIN image_storage s ON i.storage_id = s.id
         LEFT JOIN LATERAL (
           SELECT array_agg(t.name ORDER BY t.name) AS tags
           FROM image_tag it
           JOIN tag t ON it.tag_id = t.id
           WHERE it.image_id = i.id
         ) tl on TRUE
         WHERE i.id = $1
        "#,
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
        r#"
         WITH img AS (
           SELECT
             i.id, i.owner_id,
             s.size_bytes, s.width, s.height, s.mime_type, s.exif,
             i.is_public, i.created_at, i.updated_at
           FROM image i
           JOIN image_storage s ON i.storage_id = s.id
           WHERE i.owner_id = $1
           ORDER BY i.id
           LIMIT $2 OFFSET $3
         )
         SELECT img.*, COALESCE(tl.tags, '{}'::text[]) AS "tags!"
         FROM img
         LEFT JOIN LATERAL (
           SELECT array_agg(t.name ORDER BY t.name) AS tags
           FROM image_tag it
           JOIN tag t ON it.tag_id = t.id
           WHERE it.image_id = img.id
         ) tl on TRUE
         ORDER BY img.id
        "#,
        owner_id,
        pagination.limit,
        pagination.offset,
    )
    .fetch_all(pool)
    .await
    .inspect_err(|e| error!(error=?e, "fetch image records failed"))
}
