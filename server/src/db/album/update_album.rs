use super::Album;
use sqlx::PgPool;
use tracing::{error, instrument};

#[derive(Debug)]
pub struct UpdateAlbumInput<'a> {
    pub album_id: i32,
    pub name: Option<&'a str>,
    pub description: Option<&'a str>,
    pub is_public: Option<bool>,
}

pub enum UpdateAlbumResult {
    Updated(Album),
    DuplicateName,
}

#[instrument(skip(pool), fields(album_id = input.album_id))]
pub async fn update_album(
    pool: &PgPool,
    input: UpdateAlbumInput<'_>,
) -> sqlx::Result<UpdateAlbumResult> {
    let result = sqlx::query_as!(
        Album,
        "
        UPDATE album
        SET
            name = COALESCE($2, name),
            description = COALESCE($3, description),
            is_public = COALESCE($4, is_public)
        WHERE id = $1
        RETURNING *
        ",
        input.album_id,
        input.name,
        input.description,
        input.is_public,
    )
    .fetch_one(pool)
    .await;

    match result {
        Ok(album) => Ok(UpdateAlbumResult::Updated(album)),
        Err(sqlx::Error::Database(db_err)) if db_err.code() == Some("23505".into()) => {
            Ok(UpdateAlbumResult::DuplicateName)
        }
        Err(e) => Err(e),
    }
    .inspect_err(|e| error!(error=?e, "update album record failed"))
}
