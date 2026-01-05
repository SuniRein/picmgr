use crate::api::{albums, auth, create_api_doc, images, trash, user, users};
use axum::Router;
use sqlx::PgPool;
use tower_http::services::ServeDir;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

pub fn create_router(pool: PgPool) -> Router {
    let api_router = OpenApiRouter::new()
        .merge(user::create_router())
        .merge(users::create_router())
        .merge(auth::create_router())
        .merge(images::create_router())
        .merge(albums::create_router())
        .merge(trash::create_router());

    let (router, api) = OpenApiRouter::with_openapi(create_api_doc())
        .nest("/api", api_router)
        .with_state(pool)
        .split_for_parts();

    let spa = ServeDir::new("./static")
        .not_found_service(tower_http::services::ServeFile::new("./static/index.html"));

    router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api))
        .fallback_service(spa)
}
