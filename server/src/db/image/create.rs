use sqlx::PgPool;
use tracing::{error, instrument};

#[derive(Debug)]
pub struct NewImageInput<'a> {
    pub owner_id: Option<i32>,

    pub storage_key: &'a str,
    pub size_bytes: i64,
    pub width: i32,
    pub height: i32,
    pub mime_type: &'a str,
    pub exif: serde_json::Value,

    pub is_public: bool,
}

#[derive(Debug)]
pub struct CreateImageResult {
    pub image_id: i32,
    pub new_storage: bool,
}

#[instrument(skip(pool, info), fields(owner_id = info.owner_id, storage_key = info.storage_key))]
pub async fn create_image(
    pool: &PgPool,
    info: NewImageInput<'_>,
) -> sqlx::Result<CreateImageResult> {
    let mut tx = pool.begin().await?;

    let storage_row = sqlx::query!(
        r#"
        INSERT INTO image_storage (
            storage_key, size_bytes, width, height, mime_type, exif
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (storage_key) DO UPDATE
        SET storage_key = image_storage.storage_key
        RETURNING
            id,
            (xmax = 0) AS "inserted!"
        "#,
        info.storage_key,
        info.size_bytes,
        info.width,
        info.height,
        info.mime_type,
        info.exif,
    )
    .fetch_one(&mut *tx)
    .await
    .inspect_err(|e| error!(error=?e, "upsert image storage failed"))?;

    let storage_id = storage_row.id;
    let new_storage = storage_row.inserted;

    let image_id = sqlx::query_scalar!(
        r#"
        INSERT INTO image (
            storage_id, owner_id, is_public
        ) VALUES ($1, $2, $3)
        RETURNING id
        "#,
        storage_id,
        info.owner_id,
        info.is_public
    )
    .fetch_one(&mut *tx)
    .await
    .inspect_err(|e| error!(error=?e, "create image record failed"))?;

    tx.commit().await?;

    Ok(CreateImageResult {
        image_id,
        new_storage,
    })
}
