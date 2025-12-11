use super::super::{
    claims::AnyClaims,
    doc::IMAGES_TAG,
    error::{ApiError, ApiResult},
};
use crate::{db::image, image::storage::retrieve_image};
use axum::{
    body::Body,
    debug_handler,
    extract::{Path, State},
    http::{Response, header},
    response::IntoResponse,
};
use sqlx::PgPool;
use tracing::{error, info, instrument, warn};

/// Get raw image data by ID
///
/// Retrieves the raw image data for a specific image if the requester has the necessary permissions.
#[utoipa::path(
    get,
    tag = IMAGES_TAG,
    path = "/images/{id}/raw",
    security(
        (),
        ("userAuth" = []),
        ("adminAuth" = [])
    ),
    responses(
        (status = OK, description = "success response", content_type = "image/*"),
        (status = FORBIDDEN, description = "permission denied"),
        (status = NOT_FOUND, description = "image not found"),
    ),
)]
#[debug_handler]
#[instrument(skip(pool))]
pub async fn get_image(
    State(pool): State<PgPool>,
    claims: AnyClaims,
    Path(image_id): Path<i32>,
) -> ApiResult<impl IntoResponse> {
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

    let info = image::get_image_storage_info(&pool, image_id)
        .await?
        .ok_or_else(|| {
            warn!("image not found after permission check");
            ApiError::NotFound
        })?;

    let data = retrieve_image(&info.storage_key)
        .await
        .map_err(ApiError::ImageStorageError)?;

    Response::builder()
        .header(header::CONTENT_TYPE, info.mime_type)
        .body(Body::from(data))
        .inspect(|_| info!("image fetched successfully"))
        .map_err(|e| {
            error!(error=?e, "response build failed");
            ApiError::ResponseBuildError
        })
}
