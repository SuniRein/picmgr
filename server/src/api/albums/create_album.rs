use super::super::{
    claims::UserClaims,
    doc::ALBUMS_TAG,
    error::{ApiError, ApiResult},
};
use crate::db::album::{self, CreateAlbumResult, NewAlbumInput};
use axum::{Json, debug_handler, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{info, instrument};

#[derive(Deserialize, utoipa::ToSchema)]
pub struct CreateAlbumPayload {
    pub name: String,
    pub description: String,
    pub is_public: bool,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct AlbumCreateResponse {
    pub id: i32,
}

/// Create a new album
///
/// Creates a new album for the authenticated user with provided details.
#[utoipa::path(
    post,
    tag = ALBUMS_TAG,
    path = "/albums",
    security(("userAuth" = [])),
    responses(
        (status = CREATED, description = "album created successfully", body = AlbumCreateResponse),
        (status = UNAUTHORIZED, description = "invalid or missing token"),
        (status = CONFLICT, description = "album name already exists"),
    ),
)]
#[debug_handler]
#[instrument(skip(pool, payload), fields(name = %payload.name))]
pub async fn create_album(
    State(pool): State<PgPool>,
    claims: UserClaims,
    Json(payload): Json<CreateAlbumPayload>,
) -> ApiResult<impl IntoResponse> {
    let new_album = NewAlbumInput {
        owner_id: claims.user_id(),
        name: &payload.name,
        description: &payload.description,
        is_public: payload.is_public,
    };

    match album::create_album(&pool, new_album).await? {
        CreateAlbumResult::Created(album_id) => {
            info!(album_id, "album created successfully");
            Ok((
                StatusCode::CREATED,
                Json(AlbumCreateResponse { id: album_id }),
            ))
        }
        CreateAlbumResult::DuplicateName => {
            info!("album name conflict");
            Err(ApiError::AlbumNameConflict)
        }
    }
}
