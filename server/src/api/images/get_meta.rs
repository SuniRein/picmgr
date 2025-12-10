use crate::{
    api::{
        claims::{AccessClaims, AnyClaims},
        error::{ApiError, ApiResult},
        images::response::ImageMeta,
    },
    db::image,
};
use axum::{
    Json, debug_handler,
    extract::{Path, State},
};
use sqlx::PgPool;
use tracing::{info, instrument, warn};

#[debug_handler]
#[instrument(skip(pool))]
pub async fn get_image_metas(
    State(pool): State<PgPool>,
    claims: AccessClaims,
) -> ApiResult<Json<Vec<ImageMeta>>> {
    let metas = match claims {
        AccessClaims::Admin => get_all_image_metas(&pool).await?,
        AccessClaims::User(user_id) => get_user_image_metas(&pool, user_id).await?,
    };
    info!("Fetched {} images", metas.len());

    Ok(Json(metas))
}

async fn get_all_image_metas(pool: &PgPool) -> ApiResult<Vec<ImageMeta>> {
    let images = image::get_all_images(pool).await?;
    Ok(images.into_iter().map(ImageMeta::from).collect())
}

async fn get_user_image_metas(pool: &PgPool, user_id: i32) -> ApiResult<Vec<ImageMeta>> {
    let images = image::get_images_by_owner(pool, user_id).await?;
    Ok(images.into_iter().map(ImageMeta::from).collect())
}

#[debug_handler]
#[instrument(skip(pool))]
pub async fn get_image_meta(
    State(pool): State<PgPool>,
    claims: AnyClaims,
    Path(image_id): Path<i32>,
) -> ApiResult<Json<ImageMeta>> {
    let permission = image::get_image_permission(&pool, image_id)
        .await?
        .ok_or_else(|| {
            warn!("Image not found");
            ApiError::NotFound
        })?;

    if !claims.can_access_image(&permission) {
        warn!("Permission denied");
        return Err(ApiError::PermissionDenied);
    }

    let image = image::get_image_by_id(&pool, image_id)
        .await?
        .ok_or_else(|| {
            warn!("Image not found");
            ApiError::NotFound
        })?;
    info!("Fetched image successfully");
    Ok(Json(ImageMeta::from(image)))
}
