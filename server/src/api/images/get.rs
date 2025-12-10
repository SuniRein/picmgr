use crate::{
    api::{
        claims::{AccessClaims, AnyClaims},
        error::{ApiError, ApiResult},
        images::response::ImageResponse,
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
pub async fn get_images(
    State(pool): State<PgPool>,
    claims: AccessClaims,
) -> ApiResult<Json<Vec<ImageResponse>>> {
    let images = match claims {
        AccessClaims::Admin => get_all_images(&pool).await?,
        AccessClaims::User(user_id) => get_user_images(&pool, user_id).await?,
    };
    info!("Fetched {} images", images.len());

    Ok(Json(images))
}

async fn get_all_images(pool: &PgPool) -> ApiResult<Vec<ImageResponse>> {
    let images = image::get_all_images(pool).await?;
    Ok(images.into_iter().map(ImageResponse::from).collect())
}

async fn get_user_images(pool: &PgPool, user_id: i32) -> ApiResult<Vec<ImageResponse>> {
    let images = image::get_images_by_owner(pool, user_id).await?;
    Ok(images.into_iter().map(ImageResponse::from).collect())
}

#[debug_handler]
#[instrument(skip(pool))]
pub async fn get_image(
    State(pool): State<PgPool>,
    claims: AnyClaims,
    Path(image_id): Path<i32>,
) -> ApiResult<Json<ImageResponse>> {
    let image = image::get_image_by_id(&pool, image_id).await?;
    match image {
        Some(img) => {
            if img.is_public
                || matches!(claims, AnyClaims::Admin)
                || (matches!(claims, AnyClaims::User(user_id) if Some(user_id) == img.owner_id))
            {
                info!("Fetched image successfully");
                Ok(Json(ImageResponse::from(img)))
            } else {
                warn!("Permission denied");
                Err(ApiError::PermissionDenied)
            }
        }
        None => {
            warn!("Image not found");
            Err(ApiError::NotFound)
        }
    }
}
