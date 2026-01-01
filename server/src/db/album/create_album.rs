use sqlx::PgPool;
use tracing::{error, instrument};

#[derive(Debug)]
pub struct NewAlbumInput<'a> {
    pub owner_id: i32,
    pub name: &'a str,
    pub description: &'a str,
    pub is_public: bool,
}

pub enum CreateAlbumResult {
    Created(i32),
    DuplicateName,
}

#[instrument(skip(pool), fields(owner_id = input.owner_id, name = input.name))]
pub async fn create_album(
    pool: &PgPool,
    input: NewAlbumInput<'_>,
) -> sqlx::Result<CreateAlbumResult> {
    sqlx::query_scalar!(
        "
        INSERT INTO album (owner_id, name, description, is_public)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT ON CONSTRAINT album_name_unique DO NOTHING
        RETURNING id
        ",
        input.owner_id,
        input.name,
        input.description,
        input.is_public,
    )
    .fetch_optional(pool)
    .await
    .inspect_err(|e| error!(error=?e, "insert album record failed"))
    .map(|r| match r {
        Some(id) => CreateAlbumResult::Created(id),
        None => CreateAlbumResult::DuplicateName,
    })
}
