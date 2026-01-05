use sqlx::PgPool;
use tracing::{error, instrument};

#[instrument(skip(pool))]
pub async fn set_visibility(pool: &PgPool, id: i32, is_public: bool) -> sqlx::Result<()> {
    sqlx::query!(
        "UPDATE image SET is_public = $2 WHERE id = $1",
        id,
        is_public
    )
    .execute(pool)
    .await
    .inspect_err(|e| error!(error=?e, "set image visibility failed"))?;
    Ok(())
}
