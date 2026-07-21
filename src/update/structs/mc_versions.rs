use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Latest {
    release: String,
    snapshot: String,
}

impl Latest {
    pub fn release(&self)  -> &str { &self.release }
    pub fn snapshot(&self) -> &str { &self.snapshot }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    id: String,
    r#type: String,
    url: String,
    sha1: String,
}

impl Version {
    pub fn id(&self) -> &str {&self.id }
    pub fn r#type(&self) -> &str {&self.r#type }
    pub fn url(&self) -> &str {&self.url }
    pub fn sha1(&self) -> &str {&self.sha1 }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Versions {
    latest: Latest,
    versions: Vec<Version>,
}

impl Versions {
    pub fn latest(&self) -> &Latest { &self.latest }
    pub fn versions(&self) -> &[Version] { &self.versions }
    pub fn get_version_url(&self, id: &str) -> Option<&str> {
        self.versions()
            .iter()
            .find(|v| v.id() == id)
            .map(|v| v.url())
    }
}