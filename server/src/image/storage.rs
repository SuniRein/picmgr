use super::hash::hash_image;
use std::env;
use std::io;
use std::path::PathBuf;
use std::sync::LazyLock;
use tokio::fs as async_fs;
use tracing::{debug, error, instrument};

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
            .inspect_err(|e| error!(path=?dir_path, error=?e, "create directories failed"))?;
        async_fs::write(&full_path, data)
            .await
            .inspect_err(|e| error!(path=?full_path, error=?e, "write image file failed"))?;
        debug!(%hash, path=?full_path, "image stored successfully");
    } else {
        debug!(%hash, "image already exists, skipping storage");
    }

    Ok(hash)
}

#[instrument(skip(hash))]
pub async fn retrieve_image(hash: &str) -> io::Result<Vec<u8>> {
    let dir1 = &hash[0..2];
    let dir2 = &hash[2..4];
    let full_path = IMAGE_STORAGE_PATH.join(dir1).join(dir2).join(hash);

    let data = async_fs::read(&full_path)
        .await
        .inspect_err(|e| error!(path=?full_path, error=?e, "read image file failed"))?;
    debug!(%hash, path=?full_path, "image retrieved successfully");

    Ok(data)
}
