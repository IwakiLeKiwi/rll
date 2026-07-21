use serde::{Deserialize, Serialize};

use crate::update::structs::{mc_assets::AssetIndex, mc_libs::Library};

pub mod mc_assets;
pub mod mc_libs;
pub mod mc_versions;

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionManifest {
    #[serde(rename = "assetIndex")]
    asset_index: AssetIndex,
    downloads: Downloads,
    libraries: Vec<Library>,
}

impl VersionManifest {
    pub fn asset_index(&self) -> &AssetIndex { &self.asset_index }
    pub fn downloads(&self) -> &Downloads { &self.downloads }
    pub fn libraries(&self) -> &[Library] { &self.libraries }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientInfos {
    sha1: String,
    size: u64,
    url: String,
}

impl ClientInfos {
    pub fn sha1(&self) -> &str { &self.sha1 }
    pub fn size(&self) -> &u64 { &self.size }
    pub fn url(&self) -> &str { &self.url }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Downloads {
    client: ClientInfos
}

impl Downloads {
    pub fn client(&self) -> &ClientInfos { &self.client }
}