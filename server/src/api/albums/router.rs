use sqlx::PgPool;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn create_router() -> OpenApiRouter<PgPool> {
    OpenApiRouter::new()
        .routes(routes!(super::create_album::create_album))
        .routes(routes!(super::get_album::get_albums))
        .routes(routes!(super::get_album::get_album_by_id))
        .routes(routes!(super::get_stats::get_album_count))
        .routes(routes!(super::add_image::add_image_to_album))
        .routes(routes!(super::remove_image::remove_image_from_album))
        .routes(routes!(super::get_image::get_images_in_album))
        .routes(routes!(super::get_stats::get_image_count_in_album))
}
