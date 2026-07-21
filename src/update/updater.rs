pub struct Updater {
    version: String,
}

impl Updater {
    pub fn new<V: ToString>(version: V) -> Self {
        Self {
            version: version.to_string(),
        }
    }

    pub fn version(&self) -> &str { &self.version }
}