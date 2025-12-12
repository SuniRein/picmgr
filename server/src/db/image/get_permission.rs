use sqlx::{FromRow, PgPool};
use tracing::{error, instrument};

#[derive(Debug, FromRow)]
pub struct ImagePermission {
    pub owner_id: Option<i32>,
    pub is_public: bool,
}

#[instrument(skip(pool))]
pub async fn get_image_permission(pool: &PgPool, id: i32) -> sqlx::Result<Option<ImagePermission>> {
    sqlx::query_as!(
        ImagePermission,
        "SELECT owner_id, is_public FROM image WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await
    .inspect_err(|e| error!(error=?e, "fetch image permission failed"))
}
