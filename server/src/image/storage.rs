use super::hash::hash_image;
use std::env;
use std::io;
use std::path::PathBuf;
use std::sync::LazyLock;
use tokio::fs as async_fs;
use tracing::{error, info, instrument};

static IMAGE_STORAGE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    PathBuf::from(env::var("IMAGE_STORAGE_PATH").unwrap_or_else(|_| "images".to_string()))
});

#[instrument(skip(data))]
pub async fn store_image(data: &[u8]) -> io::Result<String> {
    let hash = hash_image(data);

    let dir1 = &hash[0..2];
    let dir2 = &hash[2..4];
    let dir_path = IMAGE_STORAGE_PATH.join(dir1).join(dir2);
    let full_path = dir_path.join(&hash);

    if !full_path.exists() {
        async_fs::create_dir_all(&dir_path)
            .await
            .inspect_err(|e| error!("Failed to create directories `{:?}`: {}", dir_path, e))?;
        async_fs::write(&full_path, data)
            .await
            .inspect_err(|e| error!("Failed to write image file `{:?}`: {}", full_path, e))?;
        info!("Stored new image with hash `{}` at `{:?}`", hash, full_path);
    } else {
        info!("Image with hash `{}` already exists, skipping write", hash);
    }

    Ok(hash)
}
