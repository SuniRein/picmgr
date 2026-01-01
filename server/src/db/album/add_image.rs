use sqlx::PgPool;
use tracing::{error, instrument};

#[derive(Debug)]
pub struct NewImageInput {
    pub album_id: i32,
    pub owner_id: i32,
    pub image_id: i32,
}

#[derive(Debug)]
pub enum AddImageResult {
    Added,
    AlbumNotFound,
    ImageNotFound,
    NoPermission,
    DuplicateImage,
}

#[instrument(skip(pool))]
pub async fn add_image(pool: &PgPool, input: NewImageInput) -> sqlx::Result<AddImageResult> {
    let result = sqlx::query_file_scalar!(
        "queries/add_image.sql",
        input.album_id,
        input.owner_id,
        input.image_id
    )
    .fetch_one(pool)
    .await
    .inspect_err(|e| error!(error=?e, "insert album image record failed"))?;

    match result.as_str() {
        "ok" => Ok(AddImageResult::Added),
        "album_not_found" => Ok(AddImageResult::AlbumNotFound),
        "image_not_found" => Ok(AddImageResult::ImageNotFound),
        "no_permission" => Ok(AddImageResult::NoPermission),
        "duplicate" => Ok(AddImageResult::DuplicateImage),
        o => unreachable!("unexpected result '{o}' from add_image.sql"),
    }
}
