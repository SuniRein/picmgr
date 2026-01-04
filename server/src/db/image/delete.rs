use sqlx::PgPool;
use tracing::{error, instrument};

pub enum DeleteImageResult {
    Deleted,
    NotFound,
}

/// Just removes the image meta entry.
/// Actual image storage deletion will be handled by GC periodically.
#[instrument(skip(pool))]
pub async fn delete_trashed_image(pool: &PgPool, id: i32) -> sqlx::Result<DeleteImageResult> {
    let rows = sqlx::query!(
        "
        DELETE FROM image
        WHERE trashed_at IS NOT NULL
          AND id = $1
        ",
        id
    )
    .execute(pool)
    .await
    .inspect_err(|e| error!(error=?e, "delete image meta failed"))?
    .rows_affected();

    if rows == 0 {
        Ok(DeleteImageResult::NotFound)
    } else {
        Ok(DeleteImageResult::Deleted)
    }
}

/// Deletes all trashed images for a specific user.
/// Actual image storage deletion will be handled by GC periodically.
#[instrument(skip(pool))]
pub async fn delete_all_user_trashed_images(pool: &PgPool, user_id: i32) -> sqlx::Result<u64> {
    let rows = sqlx::query!(
        "
        DELETE FROM image
        WHERE trashed_at IS NOT NULL
          AND owner_id = $1
        ",
        user_id
    )
    .execute(pool)
    .await
    .inspect_err(|e| error!(error=?e, "delete all user trashed images meta failed"))?
    .rows_affected();

    Ok(rows)
}
