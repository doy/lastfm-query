use directories::ProjectDirs;

pub fn dbpath() -> failure::Fallible<std::path::PathBuf> {
    Ok(ProjectDirs::from("", "", "lastfm-query")
        .ok_or_else(|| failure::err_msg("couldn't determine data directory"))?
        .data_dir()
        .join("tracks.sqlite"))
}
