use super::super::{
    claims::{AccessClaims, AnyClaims},
    doc::IMAGES_TAG,
    error::{ApiError, ApiResult},
    images::response::ImageMeta,
};
use crate::db::image;
use axum::{
    Json, debug_handler,
    extract::{Path, State},
};
use sqlx::PgPool;
use tracing::{info, instrument, warn};

/// Get metadata for all images
///
/// Retrieves metadata for all images if the requester has admin access.
/// For regular users, retrieves metadata only for images they own.
#[utoipa::path(
    get,
    tag = IMAGES_TAG,
    path = "/images",
    security(
        ("userAuth" = []),
        ("adminAuth" = [])
    ),
    responses(
        (status = OK, description = "success response", body = [ImageMeta]),
        (status = FORBIDDEN, description = "permission denied"),
    ),
)]
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
    info!(count = metas.len(), "images fetched");

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

/// Get image metadata by ID
///
/// Retrieves metadata for a specific image if the requester has the necessary permissions.
#[utoipa::path(
    get,
    tag = IMAGES_TAG,
    path = "/images/{id}",
    security(
        (),
        ("userAuth" = []),
        ("adminAuth" = [])
    ),
    responses(
        (status = OK, description = "success response", body = ImageMeta),
        (status = FORBIDDEN, description = "permission denied"),
        (status = NOT_FOUND, description = "image not found"),
    ),
)]
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
            info!("image not found");
            ApiError::NotFound
        })?;

    if !claims.can_access_image(&permission) {
        info!("permission denied");
        return Err(ApiError::PermissionDenied);
    }

    image::get_image_by_id(&pool, image_id)
        .await?
        .map(|image| {
            info!("image fetched successfully");
            Json(ImageMeta::from(image))
        })
        .ok_or_else(|| {
            warn!("image not found after permission check");
            ApiError::NotFound
        })
}
