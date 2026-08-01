use crate::update::{downloader::Downloader, utils::get_relative_local_dir_path};

pub struct Updater {
    version: String,
    local_dir_path: String,
}

impl Updater {
    pub fn new<V: ToString>(version: V) -> Self {
        Self {
            version: version.to_string(),
            local_dir_path: "".to_string(),
        }
    }

    pub fn install_files(&mut self) {
        println!("----- Installation of files -----");

        let (v_manifest, v_manifest_url) = self
            .get_version_manifest()
            .unwrap();

        let asset_manifest = self
            .get_assets_manifest(v_manifest.asset_index().url())
            .unwrap();

        let runtime = tokio::runtime::Runtime::new().unwrap();
        let downloader = Downloader::new(self.local_dir_path.clone());

        runtime.block_on(async {
        
            downloader.download_libraries(
                v_manifest.libraries()
            ).await;
            downloader.download_assets(
                asset_manifest.objects()
            ).await;
            downloader.download_game_files(
                &v_manifest,
                &v_manifest_url
            ).await;
        });

        println!("----- Installation completed -----");
    }

    pub fn version(&self) -> &str { &self.version }

    pub fn local_dir_path(&self) -> &str { &self.local_dir_path }

    pub fn set_local_dir_path(&mut self, local_dir_path: String) {
        self.local_dir_path = local_dir_path
    }

    pub fn set_relative_local_dir_path(&mut self, relative_local_dir_path: &str) {
        let path = get_relative_local_dir_path(relative_local_dir_path);
        self.set_local_dir_path(
            path
        );
    }
}