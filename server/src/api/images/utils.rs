use super::super::{
    claims::AnyClaims,
    error::{ApiError, ApiResult},
};
use crate::db::image;
use sqlx::PgPool;
use tracing::{info, warn};

pub(super) async fn get_image_info<F, Info>(
    query_by_id: F,
    pool: &PgPool,
    claims: AnyClaims,
    image_id: i32,
) -> ApiResult<Info>
where
    F: AsyncFn(&PgPool, i32) -> sqlx::Result<Option<Info>>,
{
    if let AnyClaims::Admin = claims {
        return query_by_id(pool, image_id).await?.ok_or_else(|| {
            info!("image not found");
            ApiError::NotFound
        });
    }

    let permission = image::get_image_permission(pool, image_id)
        .await?
        .ok_or_else(|| {
            info!("image not found");
            ApiError::NotFound
        })?;

    if !permission.is_public
        && !matches!(claims, AnyClaims::User(user_id) if permission.owner_id == Some(user_id))
    {
        info!("permission denied");
        return Err(ApiError::PermissionDenied);
    }

    query_by_id(pool, image_id).await?.ok_or_else(|| {
        warn!("image not found after permission check");
        ApiError::NotFound
    })
}
