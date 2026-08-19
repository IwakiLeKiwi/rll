mod update;

#[cfg(test)]
mod test {

    use crate::{update::updater::Updater};

    #[test]
    fn test_update() {
        let mut updater = Updater::new("1.20.1");
        updater.set_relative_local_dir_path(".rll");
        updater.install_files();
    }
}