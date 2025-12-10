use crate::{
    api::{
        error::ApiResult,
        images::response::ImageResponse,
        jwt::{AdminClaims, UserClaims},
    },
    db::image,
};
use axum::{Json, debug_handler, extract::State};
use axum_extra::either::Either;
use sqlx::PgPool;
use tracing::{Span, field::Empty, info, instrument};

#[debug_handler]
#[instrument(skip_all, fields(role = Empty, user_id = Empty))]
pub async fn get_images(
    State(pool): State<PgPool>,
    claims: Either<AdminClaims, UserClaims>,
) -> ApiResult<Json<Vec<ImageResponse>>> {
    let images = match claims {
        Either::E1(_admin) => {
            Span::current().record("role", "admin");
            get_all_images(&pool).await?
        }
        Either::E2(user) => {
            Span::current()
                .record("role", "user")
                .record("user_id", user.user_id());
            get_user_images(&pool, user.user_id()).await?
        }
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
