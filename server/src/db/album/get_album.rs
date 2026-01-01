use super::super::pagination::DbPagination;
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{FromRow, PgPool};
use tracing::{error, instrument};

#[derive(Debug, FromRow, Serialize, utoipa::ToSchema)]
pub struct Album {
    pub id: i32,
    pub owner_id: i32,

    pub name: String,
    pub description: String,

    pub is_public: bool,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[instrument(skip(pool))]
pub async fn get_album_by_id(pool: &PgPool, id: i32) -> sqlx::Result<Option<Album>> {
    sqlx::query_as!(Album, "SELECT * FROM album WHERE id = $1", id)
        .fetch_optional(pool)
        .await
        .inspect_err(|e| error!(error=?e, "fetch album record failed"))
}

#[instrument(skip(pool))]
pub async fn get_albums_by_owner(
    pool: &PgPool,
    owner_id: i32,
    pagination: DbPagination,
) -> sqlx::Result<Vec<Album>> {
    sqlx::query_as!(
        Album,
        "
        SELECT * FROM album
        WHERE owner_id = $3
        ORDER BY id
        LIMIT $1 OFFSET $2
        ",
        pagination.limit,
        pagination.offset,
        owner_id
    )
    .fetch_all(pool)
    .await
    .inspect_err(|e| error!(error=?e, "fetch album records failed"))
}
