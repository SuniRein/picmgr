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
    image::{
        ThumbnailSize, generate_thumbnail, retrieve_image, retrieve_thumbnail, store_thumbnail,
    },
};
use axum::{
    body::Body,
    debug_handler,
    extract::{Path, Query, State},
    http::header,
    response::{IntoResponse, Response},
};
use chrono::Utc;
use sqlx::PgPool;
use tracing::{debug, error, info, instrument};

/// Get a thumbnail by ID and size
///
/// Retrieves the thumbnail for a specific image if the requester has the necessary permissions.
#[utoipa::path(
    get,
    tag = IMAGES_TAG,
    path = "/images/{id}/thumbnails/{size}",
    security(
        (),
        ("userAuth" = []),
        ("adminAuth" = [])
    ),
    responses(
        (status = OK, description = "success response", content_type = "image/webp"),
        (status = FORBIDDEN, description = "permission denied"),
        (status = NOT_FOUND, description = "image not found"),
    ),
)]
#[debug_handler]
#[instrument(skip(pool))]
pub async fn get_thumbnail(
    State(pool): State<PgPool>,
    claims: AnyClaims,
    Path((image_id, size)): Path<(i32, ThumbnailSize)>,
) -> ApiResult<impl IntoResponse> {
    let info = get_image_info(image::get_image_storage_info, &pool, claims, image_id).await?;
    read_or_create_thumbnail(pool, image_id, info, size).await
}

/// Get a thumbnail by ID and size with signed query
///
/// Retrieves the thumbnail for a specific image using a signed query for authentication.
#[utoipa::path(
    get,
    tag = IMAGES_TAG,
    path = "/images/{id}/thumbnails/{size}/signed",
    params(SignedQuery),
    responses(
        (status = OK, description = "success response", content_type = "image/webp"),
        (status = FORBIDDEN, description = "permission denied"),
        (status = NOT_FOUND, description = "image not found"),
    ),
)]
#[debug_handler]
#[instrument(skip(pool))]
pub async fn get_thumbnail_signed(
    State(pool): State<PgPool>,
    Query(signed): Query<SignedQuery>,
    Path((image_id, size)): Path<(i32, ThumbnailSize)>,
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
    read_or_create_thumbnail(pool, image_id, info, size).await
}

async fn read_or_create_thumbnail(
    pool: PgPool,
    id: i32,
    storage: ImageStorageInfo,
    size: ThumbnailSize,
) -> ApiResult<impl IntoResponse> {
    let thumbnail = if storage.has_thumbnail(size) {
        retrieve_thumbnail(&storage.storage_key, size).await?
    } else {
        generate_and_store_thumbnail(&pool, id, &storage, size).await?
    };

    Ok(Response::builder()
        .header(header::CONTENT_TYPE, "image/webp")
        .body(Body::from(thumbnail))
        .inspect(|_| info!("thumbnail fetched"))
        .inspect_err(|e| error!(error=?e, "response build failed"))?)
}

async fn generate_and_store_thumbnail(
    pool: &PgPool,
    id: i32,
    storage: &ImageStorageInfo,
    size: ThumbnailSize,
) -> ApiResult<Vec<u8>> {
    debug!("thumbnail not found, generating...");

    let img = retrieve_image(&storage.storage_key).await?;
    let thumbnail = generate_thumbnail(&img, size)?;
    store_thumbnail(&thumbnail, &storage.storage_key, size).await?;

    image::set_thumbnail_exists(pool, id, size).await?;

    Ok(thumbnail)
}
