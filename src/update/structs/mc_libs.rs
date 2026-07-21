use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LibraryArtifact {
    path: String,
    sha1: String,
    size: u64,
    url: String,
}

impl LibraryArtifact {
    pub fn path(&self) -> &str { &self.path }
    pub fn sha1(&self) -> &str { &self.sha1 }
    pub fn size(&self) -> &u64 { &self.size }
    pub fn url(&self) -> &str { &self.url }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LibraryDownloads {
    artifact: LibraryArtifact 
}

impl LibraryDownloads {
    pub fn artifact(&self) -> &LibraryArtifact { &self.artifact }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Library {
    downloads: LibraryDownloads,
    name: String,
    //TODO rules
}

impl Library {
    pub fn downloads(&self) -> &LibraryDownloads { &self.downloads }
    pub fn name(&self) -> &str { &self.name }
}