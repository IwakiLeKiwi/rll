use crate::update::{structs::{VersionManifest, mc_assets::AssetRoot, mc_versions::Versions}, updater::Updater};

mod update;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("Hello from RustLauncherLib!");

    let updater = Updater::new("1.20.1");
    
    let client = reqwest::Client::new();

    let manifest: Versions = client
        .get("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json")
        .send()
        .await?
        .json()
        .await?;

    let url = manifest.get_version_url(updater.version()).expect("msg");

    let v_manifest: VersionManifest = client
        .get(url)
        .send()
        .await?
        .json()
        .await?;

    let asset_manifest: AssetRoot = client
        .get(v_manifest.asset_index().url())
        .send()
        .await?
        .json()
        .await?;

    println!("Client URL          : {}", v_manifest.downloads().client().url());
    println!("Nombre de libraries : {}", v_manifest.libraries().len());
    println!("Nombre d'assets     : {}", asset_manifest.objects().len());

    Ok(())
}