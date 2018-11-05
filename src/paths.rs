use directories::ProjectDirs;

pub fn dbpath() -> std::path::PathBuf {
    ProjectDirs::from("", "", "lastfm-query")
        .expect("coudln't determine data directory")
        .data_dir()
        .join("tracks.sqlite")
}
