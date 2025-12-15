use super::{
    super::{claims::AnyClaims, doc::IMAGES_TAG, error::ApiResult},
    utils::get_image_info,
};
use crate::{
    db::image::{self, ImageStorageInfo},
    image::{
        ThumbnailSize, generate_thumbnail, retrieve_image, retrieve_thumbnail, store_thumbnail,
    },
};
use axum::{
    body::Body, debug_handler, extract::Path, extract::State, http::header, response::IntoResponse,
    response::Response,
};
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
    let thumbenail = read_or_create_thumbnail(&pool, image_id, &info, size).await?;

    Ok(Response::builder()
        .header(header::CONTENT_TYPE, "image/webp")
        .body(Body::from(thumbenail))
        .inspect(|_| info!("thumbnail fetched"))
        .inspect_err(|e| error!(error=?e, "response build failed"))?)
}

async fn read_or_create_thumbnail(
    pool: &PgPool,
    id: i32,
    storage: &ImageStorageInfo,
    size: ThumbnailSize,
) -> ApiResult<Vec<u8>> {
    let thumbnail_exist = match size {
        ThumbnailSize::Small => storage.has_small_thumbnail,
        ThumbnailSize::Medium => storage.has_medium_thumbnail,
        ThumbnailSize::Large => storage.has_large_thumbnail,
    };

    if !thumbnail_exist {
        debug!("thumbnail not found, generating...");

        let img = retrieve_image(&storage.storage_key).await?;
        let thumbnail = generate_thumbnail(&img, size)?;
        store_thumbnail(&thumbnail, &storage.storage_key, size).await?;

        image::set_thumbnail_exists(pool, id, size).await?;

        return Ok(thumbnail);
    }

    Ok(retrieve_thumbnail(&storage.storage_key, size).await?)
}
