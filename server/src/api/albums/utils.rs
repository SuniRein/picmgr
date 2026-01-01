use super::super::error::{ApiError, ApiResult};
use crate::db::album;
use sqlx::PgPool;
use tracing::info;

pub(super) async fn check_permission(pool: &PgPool, album_id: i32, user_id: i32) -> ApiResult<()> {
    let permission = album::get_album_permission(pool, album_id)
        .await?
        .ok_or_else(|| {
            info!("album not found");
            ApiError::NotFound
        })?;

    if !permission.is_public && permission.owner_id != Some(user_id) {
        info!("permission denied");
        return Err(ApiError::PermissionDenied);
    }

    Ok(())
}
