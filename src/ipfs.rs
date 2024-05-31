use crate::config::Config;
use anyhow::{bail, Result};
use pinata_sdk::{PinByFile, PinataApi};
use reqwest::Client;
use std::path::Path;
use tokio::fs::{create_dir_all, File};
use tokio::io::AsyncWriteExt;

pub async fn load_to_ipfs(file_path: &str) -> Result<String> {
    println!("Loading to IPFS...");

    let api = PinataApi::new(
        Config::get_pinata_api_key(),
        Config::get_pinata_secret_api_key(),
    )
    .unwrap();

    let result = api.pin_file(PinByFile::new(file_path)).await;

    if let Ok(pinned_object) = result {
        println!("IPFS hash: {}", pinned_object.ipfs_hash);

        Ok(pinned_object.ipfs_hash)
    } else {
        bail!("Failed to pin object: {:?}", result.unwrap_err());
    }
}

pub async fn download_from_ipfs(ipfs_hash: &str, file_path: &str) -> Result<()> {
    if let Some(parent) = Path::new(file_path).parent() {
        create_dir_all(parent).await?;
    }

    let url = format!("{}{}", Config::ipfs_prefix(), ipfs_hash);

    println!("Downloading from IPFS: {}", url);

    let client = Client::new();
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let mut dest = File::create(file_path).await?;
        let content = response.bytes().await?;

        dest.write_all(&content).await?;

        println!("OK");

        Ok(())
    } else {
        bail!("Failed to download from IPFS: {:?}", response.status());
    }
}
