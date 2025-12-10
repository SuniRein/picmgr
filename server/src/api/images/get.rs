use crate::{
    api::{claims::AccessClaims, error::ApiResult, images::response::ImageResponse},
    db::image,
};
use axum::{Json, debug_handler, extract::State};
use sqlx::PgPool;
use tracing::{info, instrument};

#[debug_handler]
#[instrument(skip(pool))]
pub async fn get_images(
    State(pool): State<PgPool>,
    claims: AccessClaims,
) -> ApiResult<Json<Vec<ImageResponse>>> {
    let images = match claims {
        AccessClaims::Admin => get_all_images(&pool).await?,
        AccessClaims::User(user_id) => get_user_images(&pool, user_id).await?,
    };
    info!("Fetched {} images", images.len());

    Ok(Json(images))
}

async fn get_all_images(pool: &PgPool) -> ApiResult<Vec<ImageResponse>> {
    let images = image::get_all_images(pool).await?;
    Ok(images.into_iter().map(ImageResponse::from).collect())
}

async fn get_user_images(pool: &PgPool, user_id: i32) -> ApiResult<Vec<ImageResponse>> {
    let images = image::get_images_by_owner(pool, user_id).await?;
    Ok(images.into_iter().map(ImageResponse::from).collect())
}
