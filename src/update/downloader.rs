use std::{collections::HashMap, path::Path, sync::Arc};

use crate::update::structs::{mc_assets::Objects, mc_libs::Library};

pub struct Downloader {

}

impl Downloader {
    
}

pub async fn download_libraries(libs: &[Library]) {
        println!("----- Downloading libraries -----");
        let client = Arc::new(reqwest::Client::new());
        for library in libs {
            let path = format!("dl/libraries/{}", library.downloads().artifact().path());
            if let Err(e) = download_files(
                client.clone(),
                library.downloads().artifact().url().to_string(),
                &path
            ).await
            {
                println!("{e}")
            }
        }
    }

pub async fn download_assets(assets: &HashMap<String, Objects>) {
    println!("----- Downloading assets -----");
    
    for asset in assets {
        let url = format!(
            "https://resources.download.minecraft.net/{}/{}",
            &asset.1.hash()[0..2],
            &asset.1.hash()
        );
        println!("{}", url);
    }
}

pub async fn download_files(
    client: Arc<reqwest::Client>,
    url: String,
    path: &str
) -> Result<(), String> {
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to send GET request to {e}"))?;

    let bytes = response
        .bytes()
        .await
        .map_err(|_| format!("Failed to get bytes from {}", url))?;

    if let Some(parent) = Path::new(path).parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| format!("Failed to create dirs for {path}: {e}"))?;
    }

    tokio::fs::write(path, bytes)
        .await
        .map_err(|e| format!("Failed to write file {path}: {e}"))?;

    Ok(())
}