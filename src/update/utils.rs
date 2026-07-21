pub fn get_relative_local_dir_path(local_dir_path: &str) -> String {
    #[cfg(unix)]
    {
        let root = std::env::var("HOME").expect("No HOME directory");
        root + "/" + local_dir_path + "/"
    }

    #[cfg(windows)]
    {
        let root = std::env::var("APPDATA").expect("No APPDATA directory");
        root + "\\" + local_dir_path + "\\"
    }
}