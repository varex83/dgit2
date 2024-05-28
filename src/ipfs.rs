use crate::config::Config;
use anyhow::{bail, Result};
use pinata_sdk::{PinByFile, PinataApi};

pub async fn load_to_ipfs(file_path: &str) -> Result<String> {
    println!("Loading to IPFS...");

    let api = PinataApi::new(
        Config::get_pinata_api_key(),
        Config::get_pinata_secret_api_key(),
    )
    .unwrap();

    let result = api.pin_file(PinByFile::new(file_path)).await;

    if let Ok(pinned_object) = result {
        Ok(pinned_object.ipfs_hash)
    } else {
        bail!("Failed to pin object: {:?}", result.unwrap_err());
    }
}
