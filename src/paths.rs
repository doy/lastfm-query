pub fn db_path() -> failure::Fallible<std::path::PathBuf> {
    Ok(directories::ProjectDirs::from("", "", "lastfm-query")
        .ok_or_else(|| failure::err_msg("couldn't determine data directory"))?
        .data_dir()
        .join("tracks.sqlite"))
}
