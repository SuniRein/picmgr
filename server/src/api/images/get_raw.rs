use super::{
    super::{
        claims::AnyClaims,
        doc::IMAGES_TAG,
        error::{ApiError, ApiResult},
    },
    sign::SignedQuery,
    utils::get_image_info,
};
use crate::{
    db::image::{self, ImageStorageInfo},
    image::retrieve_image,
};
use axum::{
    body::Body,
    debug_handler,
    extract::{Path, Query, State},
    http::{Response, header},
    response::IntoResponse,
};
use chrono::Utc;
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
    read_image(info).await
}

/// Get raw image data by ID with signed query
///
/// Retrieves the raw image data for a specific image using a signed query for authentication.
#[utoipa::path(
    get,
    tag = IMAGES_TAG,
    path = "/images/{id}/raw/signed",
    params(SignedQuery),
    responses(
        (status = OK, description = "success response", content_type = "image/*"),
        (status = FORBIDDEN, description = "permission denied"),
        (status = NOT_FOUND, description = "image not found"),
    ),
)]
#[debug_handler]
#[instrument(skip(pool))]
pub async fn get_image_signed(
    State(pool): State<PgPool>,
    Query(signed): Query<SignedQuery>,
    Path(image_id): Path<i32>,
) -> ApiResult<impl IntoResponse> {
    signed.verify(image_id, Utc::now()).inspect_err(|e| {
        info!(cause=?e, "signed query verification failed");
    })?;

    let info = image::get_image_storage_info(&pool, image_id)
        .await?
        .ok_or_else(|| {
            info!("image not found");
            ApiError::NotFound
        })?;

    read_image(info).await
}

async fn read_image(info: ImageStorageInfo) -> ApiResult<impl IntoResponse> {
    let data = retrieve_image(&info.storage_key).await?;
    Ok(Response::builder()
        .header(header::CONTENT_TYPE, info.mime_type)
        .body(Body::from(data))
        .inspect(|_| info!("image fetched"))
        .inspect_err(|e| error!(error=?e, "response build failed"))?)
}
