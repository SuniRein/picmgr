use super::super::{
    claims::UserClaims,
    doc::ALBUMS_TAG,
    error::{ApiError, ApiResult},
};
use crate::db::album::{self, AddImageResult, NewImageInput};
use axum::{
    Json, debug_handler,
    extract::{Path, State},
};
use serde::Deserialize;
use sqlx::PgPool;
use tracing::{info, instrument};

#[derive(Deserialize, utoipa::ToSchema)]
pub struct AddImagePayload {
    pub image_id: i32,
}

/// Add an image to an album
///
/// Adds an existing image to a specified album for the authenticated user.
/// The image will be pushed to the end of the album's image list.
#[utoipa::path(
    post,
    tag = ALBUMS_TAG,
    path = "/albums/{album_id}/images",
    security(("userAuth" = [])),
    responses(
        (status = OK, description = "image added to album successfully"),
        (status = NOT_FOUND, description = "album or image not found"),
        (status = FORBIDDEN, description = "permission denied"),
        (status = CONFLICT, description = "image already in album"),
        (status = UNAUTHORIZED, description = "invalid or missing token"),
    ),
)]
#[debug_handler]
#[instrument(skip(pool, payload), fields(image_id = %payload.image_id))]
pub async fn add_image_to_album(
    State(pool): State<PgPool>,
    claims: UserClaims,
    Path(album_id): Path<i32>,
    Json(payload): Json<AddImagePayload>,
) -> ApiResult<()> {
    let input = NewImageInput {
        album_id,
        owner_id: claims.user_id(),
        image_id: payload.image_id,
    };

    match album::add_image(&pool, input).await? {
        AddImageResult::Added => {
            info!("image added to album successfully");
            Ok(())
        }
        AddImageResult::AlbumNotFound => {
            info!("album not found");
            Err(ApiError::NotFound)
        }
        AddImageResult::ImageNotFound => {
            info!("image not found");
            Err(ApiError::NotFound)
        }
        AddImageResult::NoPermission => {
            info!("permission denied");
            Err(ApiError::PermissionDenied)
        }
        AddImageResult::DuplicateImage => {
            info!("image already in album");
            Err(ApiError::ImageAlreadyInAlbum)
        }
    }
}
