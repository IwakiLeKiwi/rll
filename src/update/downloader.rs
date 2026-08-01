use std::{collections::HashMap, path::Path, sync::Arc};

use colored::Colorize;
use sha1::{Digest, Sha1};

use crate::update::structs::{VersionManifest, mc_assets::Objects, mc_libs::Library};

pub struct Downloader {
    local_dir_path: String,
    client: Arc<reqwest::Client>,
}

impl Default for Downloader {
    fn default() -> Self {
        Self { 
            local_dir_path: String::new(),
            client: Arc::new(reqwest::Client::new()),
        }
    }
}

impl Downloader {
    
    pub async fn download_libraries(&self, libs: &[Library]) {
        println!("----- Downloading libraries -----");
        for library in libs {

            let Some(artifact) = library.downloads().artifact() else {
                continue;
            };

            let path = format!("{}/libraries/{}", self.local_dir_path, artifact.path());
            let expected_hash = artifact.sha1();

            if file_matches_hash(&path, expected_hash).await {
                println!("Skipping {} ! (up to date)", library.name().green().italic());
                continue;
            }

            println!("Downloading {}", library.name().blue().bold());
            if let Err(e) = download_files(
                self.client.clone(),
                artifact.url().to_string(),
                &path).await {
                println!("{e}")
            }
        }
    }

    pub async fn download_assets(&self, assets: &HashMap<String, Objects>) {
        println!("----- Downloading assets -----");
        
        for asset in assets {

            let url = format!(
                "https://resources.download.minecraft.net/{}/{}",
                &asset.1.hash()[0..2],
                &asset.1.hash()
            );
            let path = format!("{}/assets/objects/{}/{}",
                self.local_dir_path,
                &asset.1.hash()[0..2],
                &asset.1.hash()
            );
            let expected_hash = asset.1.hash();

            if file_matches_hash(&path, expected_hash).await {
                println!("Skipping {} ! (up to date)", asset.0.green().italic());
                continue;
            }

            println!("Downloading {}", asset.0.blue().bold());
            if let Err(e) = download_files(
                self.client.clone(),
                url,
                &path).await {
                println!("{e}")
            }
        }
    }

    pub async fn download_game_files(&self, version_manifest: &VersionManifest, version_url: &str) {
        println!("----- Downloading game files -----");
        let version = version_manifest.id();

        /* Download the client.jar */
        let jar_path = format!("{}/versions/{}/{}.jar", self.local_dir_path, version, version);
        let jar_url = version_manifest.downloads().client().url();
        let jar_sha1 = version_manifest.downloads().client().sha1();

        if file_matches_hash(&jar_path, jar_sha1).await {
            println!("Skipping {} (up to date)", format!("{version}.jar").green().italic());
        } else {
            println!("Downloading {}", format!("{version}.jar").blue().bold());
            if let Err(e) = download_files(
                self.client.clone(),
                jar_url.to_string(),
                &jar_path).await {
                println!("{e}")
            }
        }

        let json_path = format!("{}/versions/{}/{}.json",
            self.local_dir_path,
            version,
            version);

        /* Download version.json */
        if tokio::fs::metadata(&json_path).await.is_ok() {
            println!("Skipping {} (up to date)", format!("{version}.json").green().italic());
        } else {
            println!("Downloading {}", format!("{version}.json").blue().bold());
            if let Err(e) = download_files(
                self.client.clone(),
                version_url.to_string(),
                &json_path).await {
                println!("{e}")
            }
        }

        let index_path = format!("{}/assets/indexes/{}.json", self.local_dir_path, version_manifest.asset_index().id());
        let index_hash = version_manifest.asset_index().sha1();

        if file_matches_hash(&index_path, index_hash).await {
            println!("Skipping {} (up to date)", format!("{}.json", version_manifest.asset_index().id()).green().italic());
        } else {
            println!("Downloading {}", format!("{}.json", version_manifest.asset_index().id()).blue().bold());
            if let Err(e) = download_files(
                self.client.clone(), 
                version_manifest.asset_index().url().to_string(), 
                &index_path).await {
                println!("{e}");
            }
        }
    }

    pub fn new(path: String) -> Self {
        Self { 
            local_dir_path: path,
            client: Arc::new(reqwest::Client::new()),
            ..Self::default()
        }
    }

    pub fn client(&self) -> &Arc<reqwest::Client> { &self.client }
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
        .map_err(|e| format!("Failed to send GET request to {url} : {e}"))?;

    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to get bytes from {url} : {e}"))?;

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

pub async fn file_matches_hash(path: &str, expected_hash: &str) -> bool {
    let Ok(bytes) = tokio::fs::read(path).await else {
        return false;
    };

    let hash = Sha1::digest(&bytes)
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();
    hash == expected_hash
}