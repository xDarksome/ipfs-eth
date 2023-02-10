//! IPFS related capabilities of this library.

use std::path::PathBuf;

use ipfs_api::{IpfsApi as _, IpfsClient};
use thiserror::Error;

/// Uploads the provided file to the IPFS network returning its CID.
///
/// # Errors
///
/// See [`UploadFileError`] for details.
pub async fn upload_file(path: PathBuf) -> Result<String, UploadFileError> {
    let file = std::fs::File::open(path)?;
    Ok(IpfsClient::default().add(file).await?.hash)
}

#[derive(Error, Debug)]
pub enum UploadFileError {
    #[error("Failed to open the file: {0}")]
    OpenFile(#[from] std::io::Error),

    #[error("IPFS /add request failed: {0}")]
    IpfsAdd(#[from] ipfs_api::Error),
}
