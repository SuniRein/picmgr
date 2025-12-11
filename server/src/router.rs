use crate::api::{auth, create_api_doc, images, user, users};
use axum::Router;
use sqlx::PgPool;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

pub fn create_router(pool: PgPool) -> Router {
    let api_router = OpenApiRouter::new()
        .routes(routes!(user::get_current_user))
        .routes(routes!(users::get_user))
        .routes(routes!(users::get_all_users))
        .merge(auth::create_auth_router())
        .merge(images::create_image_router());

    let (router, api) = OpenApiRouter::with_openapi(create_api_doc())
        .nest("/api", api_router)
        .with_state(pool)
        .split_for_parts();

    router.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api))
}
