use super::{super::pagination::DbPagination, get_meta::ImageMeta};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::PgPool;
use tracing::{error, instrument};

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct DbImageFilter {
    pub min_width: Option<i32>,
    pub max_width: Option<i32>,
    pub min_height: Option<i32>,
    pub max_height: Option<i32>,
    pub mime_types: Option<String>,
    pub created_before: Option<DateTime<Utc>>,
    pub created_after: Option<DateTime<Utc>>,
    pub updated_before: Option<DateTime<Utc>>,
    pub updated_after: Option<DateTime<Utc>>,
    pub is_public: Option<bool>,
    pub owner_id: Option<i32>,
    pub album_id: Option<i32>,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[instrument(skip(pool, filter))]
pub async fn get_filtered_image_metas(
    pool: &PgPool,
    filter: DbImageFilter,
    pagination: DbPagination,
) -> sqlx::Result<Vec<ImageMeta>> {
    sqlx::query_file_as!(
        ImageMeta,
        "queries/search_image.sql",
        pagination.limit,
        pagination.offset,
        filter.min_width,
        filter.max_width,
        filter.min_height,
        filter.max_height,
        filter.mime_types,
        filter.created_before,
        filter.created_after,
        filter.updated_before,
        filter.updated_after,
        filter.is_public,
        filter.owner_id,
        filter.album_id,
        &filter.tags,
    )
    .fetch_all(pool)
    .await
    .inspect_err(|e| error!(error=?e, "search image records failed"))
}

#[instrument(skip(pool, filter))]
pub async fn get_filtered_image_meta_count(
    pool: &PgPool,
    filter: DbImageFilter,
) -> sqlx::Result<i64> {
    sqlx::query_file_scalar!(
        "queries/search_image_count.sql",
        0,
        0,
        filter.min_width,
        filter.max_width,
        filter.min_height,
        filter.max_height,
        filter.mime_types,
        filter.created_before,
        filter.created_after,
        filter.updated_before,
        filter.updated_after,
        filter.is_public,
        filter.owner_id,
        filter.album_id,
        &filter.tags,
    )
    .fetch_one(pool)
    .await
    .inspect_err(|e| error!(error=?e, "count image records failed"))
}
