use sqlx::PgPool;
use tracing::{error, instrument};

#[instrument(skip(pool, tags))]
pub async fn set_image_tags(pool: &PgPool, id: i32, tags: &[String]) -> sqlx::Result<()> {
    let mut tx = pool.begin().await?;

    if !tags.is_empty() {
        sqlx::query!(
            "
             INSERT INTO tag (name)
             SELECT t FROM UNNEST($1::text[]) AS t
             ON CONFLICT (name) DO NOTHING
            ",
            tags
        )
        .execute(tx.as_mut())
        .await
        .inspect_err(|e| error!(error=?e, "insert new tags failed"))?;
    }

    let tag_ids: Vec<i32> = if !tags.is_empty() {
        sqlx::query!("SELECT id FROM tag WHERE name = ANY($1)", tags)
            .fetch_all(tx.as_mut())
            .await
            .inspect_err(|e| error!(error=?e, "fetch tags failed"))?
            .into_iter()
            .map(|r| r.id)
            .collect()
    } else {
        Vec::new()
    };

    if tag_ids.is_empty() {
        // No tags to associate, just clear existing associations
        sqlx::query!("DELETE FROM image_tag WHERE image_id = $1", id)
            .execute(tx.as_mut())
            .await
            .inspect_err(|e| error!(error=?e, "delete tags failed"))?;
    } else {
        // Clear existing associations
        sqlx::query!(
            "
             DELETE FROM image_tag
             WHERE image_id = $1
               AND tag_id NOT IN (SELECT UNNEST($2::int[]))
            ",
            id,
            &tag_ids,
        )
        .execute(tx.as_mut())
        .await
        .inspect_err(|e| error!(error=?e, "delete existing tags failed"))?;

        // Insert new associations
        sqlx::query!(
            "
             INSERT INTO image_tag (image_id, tag_id)
             SELECT $1, t.id FROM UNNEST($2::int[]) AS t(id)
             ON CONFLICT (image_id, tag_id) DO NOTHING
            ",
            id,
            &tag_ids
        )
        .execute(tx.as_mut())
        .await
        .inspect_err(|e| error!(error=?e, "insert image tags failed"))?;
    }

    tx.commit().await?;

    Ok(())
}
