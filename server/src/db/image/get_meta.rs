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

    tags: Vec<String>,

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
        r#"
         WITH img AS (
           SELECT
             id, owner_id, category_id, size_bytes, width, height, mime_type, exif,
             is_public, created_at, updated_at
           FROM image i
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
          id, owner_id, category_id, size_bytes, width, height, mime_type, exif,
          is_public, created_at, updated_at,
          COALESCE(tl.tags, '{}'::text[]) AS "tags!"
         FROM image i
         LEFT JOIN LATERAL (
           SELECT array_agg(t.name ORDER BY t.name) AS tags
           FROM image_tag it
           JOIN tag t ON it.tag_id = t.id
           WHERE it.image_id = i.id
         ) tl on TRUE
         WHERE id = $1
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
             id, owner_id, category_id, size_bytes, width, height, mime_type, exif,
             is_public, created_at, updated_at
           FROM image i
           WHERE owner_id = $1
           ORDER BY id
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
