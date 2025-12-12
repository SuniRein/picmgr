use super::super::{
    claims::UserClaims,
    doc::IMAGES_TAG,
    error::{ApiError, ApiResult},
};
use crate::{
    db::image::{NewImageInput, create_image},
    image::{parse::ImageInfo, storage::store_image},
};
use axum::{
    Json, body::Bytes, debug_handler, extract::State, http::StatusCode, response::IntoResponse,
};
use axum_extra::TypedHeader;
use headers::ContentType;
use sqlx::PgPool;
use tracing::{debug, info, instrument, warn};

/// Upload a raw image
///
/// Uploads a raw image file to the server, whose owner is set to the authenticated user.
#[utoipa::path(
    post,
    tag = IMAGES_TAG,
    path = "/images/upload/raw",
    security(("userAuth" = [])),
    request_body(
        content(
            ("image/png"), ("image/jpeg"), ("image/gif"), ("image/bmp"),
            ("image/x-icon"), ("image/tiff"), ("image/webp"),
        )
    ),
    responses(
        (status = CREATED, description = "image uploaded successfully"),
        (status = BAD_REQUEST, description = "image parsed error"),
        (status = UNAUTHORIZED, description = "invalid or missing token"),
        (status = UNSUPPORTED_MEDIA_TYPE, description = "unsupported image format"),
    ),
)]
#[debug_handler]
#[instrument(skip(pool, body, maybe_ct))]
pub async fn upload_raw_image(
    State(pool): State<PgPool>,
    claims: UserClaims,
    maybe_ct: Option<TypedHeader<ContentType>>,
    body: Bytes,
) -> ApiResult<impl IntoResponse> {
    match maybe_ct {
        Some(ct) => {
            let mime = ct.0.to_string();
            if mime.starts_with("image/") {
                debug!(%mime, "content type accepted");
            } else {
                warn!(%mime, "content type rejected");
            }
        }
        None => warn!("no content type provided"),
    }

    let image_info = ImageInfo::parse(&body)?;
    let storage_key = store_image(&body)
        .await
        .map_err(ApiError::ImageStorageError)?;

    let image_input = NewImageInput {
        owner_id: Some(claims.user_id()),
        category_id: None,

        storage_key: &storage_key,
        size_bytes: body.len() as i64,
        width: image_info.width as i32,
        height: image_info.height as i32,
        mime_type: &image_info.mime_type,
        exif: image_info.exif.as_ref(),

        has_small_thumbnail: false,
        has_medium_thumbnail: false,
        has_large_thumbnail: false,

        is_public: false,
    };
    let image_id = create_image(&pool, image_input).await?;
    info!(image_id, "image record created in database");

    // TODO: if create_image fails, consider deleting the stored image to avoid orphaned files

    Ok((StatusCode::CREATED, Json("Image uploaded successfully")))
}
