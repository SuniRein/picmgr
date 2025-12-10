use crate::api::{auth, images, user, users};
use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;

pub fn create_router(pool: PgPool) -> Router {
    let api = Router::new()
        .route("/auth/register", post(auth::register))
        .route("/auth/login", post(auth::login))
        .route("/auth/login/admin", post(auth::login_as_admin))
        .route("/auth/refresh", post(auth::refresh_token))
        .route("/user/me", get(user::get_current_user))
        .route("/users/{id}", get(users::get_user))
        .route("/users", get(users::get_all_users))
        .route("/images/upload/raw", post(images::upload_raw_image))
        .route("/images/{id}", get(images::get_image))
        .route("/images", get(images::get_images));

    Router::new().nest("/api", api).with_state(pool)
}
