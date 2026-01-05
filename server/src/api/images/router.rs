use axum::extract::DefaultBodyLimit;
use sqlx::PgPool;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn create_router() -> OpenApiRouter<PgPool> {
    let upload_router = OpenApiRouter::new()
        .routes(routes!(super::upload::upload_raw_image))
        .layer(DefaultBodyLimit::max(20 * 1024 * 1024));

    OpenApiRouter::new()
        .routes(routes!(super::get_stats::get_image_count))
        .routes(routes!(super::get_meta::get_image_meta))
        .routes(routes!(super::get_meta::get_image_metas))
        .routes(routes!(super::get_raw::get_image))
        .routes(routes!(super::get_raw::get_image_signed))
        .routes(routes!(super::get_thumbnail::get_thumbnail))
        .routes(routes!(super::get_thumbnail::get_thumbnail_signed))
        .routes(routes!(super::search::get_filtered_image_meta_count))
        .routes(routes!(super::search::get_filtered_image_metas))
        .routes(routes!(
            super::tag::get_image_tags,
            super::tag::set_image_tags,
        ))
        .routes(routes!(super::update_meta::update_image_meta))
        .merge(upload_router)
        .routes(routes!(super::trash::trash_image))
}
