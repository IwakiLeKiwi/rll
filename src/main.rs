use crate::update::updater::Updater;

mod update;

fn main() {

    let mut updater = Updater::new("1.20.1");
    updater.set_relative_local_dir_path(".rll");
    updater.install_files();
}