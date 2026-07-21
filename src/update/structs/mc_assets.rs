use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetIndex {
    id: String,
    sha1: String,
    size: u64,
    url: String,
}

impl AssetIndex {
    pub fn id(&self) -> &str { &self.id }
    pub fn sha1(&self) -> &str { &self.sha1 }
    pub fn size(&self) -> &u64 { &self.size }
    pub fn url(&self) -> &str { &self.url }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Objects {
    hash: String,
    size: u64,
}

impl Objects {
    pub fn hash(&self) -> &str { &self.hash }
    pub fn size(&self) -> &u64 { &self.size }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetRoot {
    objects: HashMap<String, Objects>,
}

impl AssetRoot {
    pub fn objects(&self) -> &HashMap<String, Objects> { &self.objects }
}