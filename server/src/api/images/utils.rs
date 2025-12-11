use super::super::{
    claims::AnyClaims,
    error::{ApiError, ApiResult},
};
use crate::db::image::{self, Image};
use sqlx::PgPool;
use tracing::{info, warn};

pub(super) async fn get_image_info(
    pool: &PgPool,
    claims: AnyClaims,
    image_id: i32,
) -> ApiResult<Image> {
    if let AnyClaims::Admin = claims {
        return image::get_image_by_id(pool, image_id)
            .await?
            .ok_or_else(|| {
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

    image::get_image_by_id(pool, image_id)
        .await?
        .ok_or_else(|| {
            warn!("image not found after permission check");
            ApiError::NotFound
        })
}
