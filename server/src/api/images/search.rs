use super::{
    super::{
        claims::AccessClaims,
        doc::IMAGES_TAG,
        error::ApiResult,
        pagination::{PaginatedResponse, PaginationQuery},
        stats::CountResponse,
    },
    get_meta::ImageMetaResponse,
};
use crate::db::image::{self, DbImageFilter};
use axum::{Json, debug_handler, extract::State};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::{info, instrument};

#[derive(Deserialize, utoipa::ToSchema)]
pub struct ImageSearchRequest {
    filter: DbImageFilter,
    pagination: PaginationQuery,
}

/// Search images based on various criteria
///
/// Allows users to search for images by specifying multiple optional filters.
#[utoipa::path(
    post,
    tag = IMAGES_TAG,
    path = "/images/search",
    security(
        ("userAuth" = []),
        ("adminAuth" = [])
    ),
    responses(
        (status = OK, description = "success response", body = PaginatedResponse<ImageMetaResponse>),
        (status = BAD_REQUEST, description = "invalid search parameters"),
        (status = UNAUTHORIZED, description = "invalid or missing token"),
    ),
)]
#[debug_handler]
#[instrument(skip(pool, filter))]
pub async fn get_filtered_image_metas(
    State(pool): State<PgPool>,
    claims: AccessClaims,
    Json(ImageSearchRequest {
        pagination,
        mut filter,
    }): Json<ImageSearchRequest>,
) -> ApiResult<Json<PaginatedResponse<ImageMetaResponse>>> {
    let pagination = pagination.check()?;

    if let AccessClaims::User(id) = claims {
        filter.owner_id = Some(id);
    }
    let metas = image::get_filtered_image_metas(&pool, filter, pagination.into()).await?;

    let now = Utc::now();
    let response_metas = metas
        .into_iter()
        .map(|meta| ImageMetaResponse::generate_from(meta, now))
        .collect::<Vec<_>>();
    info!(count = response_metas.len(), "image search results fetched");

    Ok(Json(PaginatedResponse::new(response_metas, pagination)))
}

/// Get count of images matching search criteria
///
/// Allows users to get the count of images matching specified filters.
#[utoipa::path(
    post,
    tag = IMAGES_TAG,
    path = "/images/search/count",
    security(
        ("userAuth" = []),
        ("adminAuth" = [])
    ),
    responses(
        (status = OK, description = "success response", body = CountResponse),
        (status = BAD_REQUEST, description = "invalid search parameters"),
        (status = UNAUTHORIZED, description = "invalid or missing token"),
    ),
)]
#[debug_handler]
#[instrument(skip(pool, filter))]
pub async fn get_filtered_image_meta_count(
    State(pool): State<PgPool>,
    claims: AccessClaims,
    Json(mut filter): Json<DbImageFilter>,
) -> ApiResult<Json<CountResponse>> {
    if let AccessClaims::User(id) = claims {
        filter.owner_id = Some(id);
    }

    let count = image::get_filtered_image_meta_count(&pool, filter).await?;
    info!(count, "image search count fetched");

    Ok(Json(CountResponse { count }))
}
