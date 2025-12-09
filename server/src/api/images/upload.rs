use crate::{
    api::{
        error::{ApiError, ApiResult},
        jwt::UserClaims,
    },
    db::image::{NewImageInput, create_image},
    image::{parse::ImageInfo, storage::store_image},
};
use axum::{
    Json, body::Bytes, debug_handler, extract::State, http::StatusCode, response::IntoResponse,
};
use axum_extra::TypedHeader;
use headers::ContentType;
use sqlx::PgPool;
use tracing::{info, instrument, warn};

#[debug_handler]
#[instrument(skip(pool, body, claims, maybe_ct), fields(user_id = claims.user_id()))]
pub async fn upload_raw_image(
    State(pool): State<PgPool>,
    claims: UserClaims,
    maybe_ct: Option<TypedHeader<ContentType>>,
    body: Bytes,
) -> ApiResult<impl IntoResponse> {
    match maybe_ct {
        Some(ct) => {
            if ct.0.to_string().starts_with("image/") {
                info!("Received valid image upload with Content-Type: {}", ct.0);
            } else {
                warn!("Invalid Content-Type for image upload: {}", ct.0);
            }
        }
        None => warn!("No Content-Type header provided"),
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

        small_thumbnail_key: None,
        medium_thumbnail_key: None,
        large_thumbnail_key: None,

        is_public: true,
    };
    let image = create_image(&pool, image_input).await?;

    info!("Image uploaded successfully with ID: {}", image.id);

    // TODO: if create_image fails, consider deleting the stored image to avoid orphaned files

    Ok((StatusCode::CREATED, Json("Image uploaded successfully")))
}
