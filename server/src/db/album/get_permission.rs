use sqlx::{FromRow, PgPool};
use tracing::{error, instrument};

#[derive(Debug, FromRow)]
pub struct AlbumPermission {
    pub owner_id: Option<i32>,
    pub is_public: bool,
}

#[instrument(skip(pool))]
pub async fn get_album_permission(pool: &PgPool, id: i32) -> sqlx::Result<Option<AlbumPermission>> {
    sqlx::query_as!(
        AlbumPermission,
        "SELECT owner_id, is_public FROM album WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await
    .inspect_err(|e| error!(error=?e, "fetch album permission failed"))
}
