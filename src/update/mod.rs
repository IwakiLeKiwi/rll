use crate::update::{structs::{VersionManifest, mc_assets::{AssetIndex, AssetRoot}, mc_versions::Versions}, updater::Updater};

pub mod structs;

pub mod downloader;
pub mod updater;
pub mod utils;

impl Updater {

    pub fn get_version_list(&self) -> Result<Versions, reqwest::Error> {
        let client = reqwest::Client::new();
        let runtime = tokio::runtime::Runtime::new().unwrap();
        
        runtime.block_on(async {
            client
                .get("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json")
                .send()
                .await?
                .json()
                .await
        })
    }

    pub fn get_version_manifest(&self) -> Result<VersionManifest, reqwest::Error> {
        let client = reqwest::Client::new();
        let runtime = tokio::runtime::Runtime::new().unwrap();

        let versions = self.get_version_list()?;
        
        let url = versions
            .get_version_url(&self.version())
            .expect("No version found!");
        
        runtime.block_on(async {
            client
                .get(url)
                .send()
                .await?
                .json()
                .await
        })
    }

    pub fn get_assets_manifest(&self, url: &str) -> Result<AssetRoot, reqwest::Error> {
        let client = reqwest::Client::new();
        let runtime = tokio::runtime::Runtime::new().unwrap();
        
        runtime.block_on(async {
            client
                .get(url)
                .send()
                .await?
                .json()
                .await
        })
    }
}