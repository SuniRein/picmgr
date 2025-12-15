use super::{
    super::{claims::AnyClaims, doc::IMAGES_TAG, error::ApiResult},
    utils::get_image_info,
};
use crate::{db::image, image::retrieve_image};
use axum::{
    body::Body,
    debug_handler,
    extract::{Path, State},
    http::{Response, header},
    response::IntoResponse,
};
use sqlx::PgPool;
use tracing::{error, info, instrument};

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
    let info = get_image_info(image::get_image_storage_info, &pool, claims, image_id).await?;
    let data = retrieve_image(&info.storage_key).await?;
    Ok(Response::builder()
        .header(header::CONTENT_TYPE, info.mime_type)
        .body(Body::from(data))
        .inspect(|_| info!("image fetched"))
        .inspect_err(|e| error!(error=?e, "response build failed"))?)
}
