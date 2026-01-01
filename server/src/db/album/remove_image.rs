use sqlx::PgPool;
use tracing::{error, instrument};

#[instrument(skip(pool))]
pub async fn remove_image_from_album(
    pool: &PgPool,
    album_id: i32,
    image_id: i32,
) -> sqlx::Result<bool> {
    let result = sqlx::query!(
        "
        DELETE FROM image_album
        WHERE album_id = $1 AND image_id = $2
        ",
        album_id,
        image_id,
    )
    .execute(pool)
    .await
    .inspect_err(|e| error!(error=?e, "remove image from album failed"))?;

    Ok(result.rows_affected() > 0)
}
