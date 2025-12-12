use crate::image::ThumbnailSize;
use utoipa::{
    OpenApi,
    openapi::{
        ComponentsBuilder, OpenApi as OpenApiDoc, OpenApiBuilder,
        security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    },
};

pub const USER_TAG: &str = "user";
pub const USERS_TAG: &str = "users";
pub const AUTH_TAG: &str = "auth";
pub const IMAGES_TAG: &str = "images";

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = USER_TAG, description = "Current user API endpoints"),
        (name = USERS_TAG, description = "User management API endpoints"),
        (name = AUTH_TAG, description = "Authentication API endpoints"),
        (name = IMAGES_TAG, description = "Image management API endpoints"),
    ),
    components(
        schemas(
            ThumbnailSize // it's strange that ThumbnailSize is not automatically included
        )
    )
)]
struct ApiDoc;

pub fn create_api_doc() -> OpenApiDoc {
    let mut doc = ApiDoc::openapi();
    doc.merge(
        OpenApiBuilder::new()
            .components(Some(
                ComponentsBuilder::new()
                    .security_scheme("adminAuth", create_admin_auth_scheme())
                    .security_scheme("userAuth", create_user_auth_scheme())
                    .security_scheme("refreshToken", create_refresh_token_auth_scheme())
                    .build(),
            ))
            .build(),
    );
    doc
}

fn create_admin_auth_scheme() -> SecurityScheme {
    SecurityScheme::Http(
        HttpBuilder::new()
            .scheme(HttpAuthScheme::Bearer)
            .bearer_format("JWT")
            .description(Some("Admin authentication using JWT tokens"))
            .build(),
    )
}

fn create_user_auth_scheme() -> SecurityScheme {
    SecurityScheme::Http(
        HttpBuilder::new()
            .scheme(HttpAuthScheme::Bearer)
            .bearer_format("JWT")
            .description(Some("User authentication using JWT tokens"))
            .build(),
    )
}

fn create_refresh_token_auth_scheme() -> SecurityScheme {
    SecurityScheme::Http(
        HttpBuilder::new()
            .scheme(HttpAuthScheme::Bearer)
            .bearer_format("JWT")
            .description(Some("Refresh token authentication using JWT tokens"))
            .build(),
    )
}
