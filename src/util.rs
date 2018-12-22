pub fn program_name() -> failure::Fallible<String> {
    let program = std::env::args()
        .next()
        .ok_or_else(|| format_err!("no program name found"))?;
    let path = std::path::Path::new(&program);
    let filename = path
        .file_name()
        .ok_or_else(|| format_err!("invalid filename found"))?
        .to_string_lossy()
        .to_string();
    Ok(filename)
}

pub fn db_path() -> failure::Fallible<std::path::PathBuf> {
    Ok(directories::ProjectDirs::from("", "", "lastfm-query")
        .ok_or_else(|| failure::err_msg("couldn't determine data directory"))?
        .data_dir()
        .join("tracks.sqlite"))
}

pub fn api_key_path() -> failure::Fallible<std::path::PathBuf> {
    Ok(directories::ProjectDirs::from("", "", "lastfm-query")
        .ok_or_else(|| {
            failure::err_msg("couldn't determine config directory")
        })?
        .config_dir()
        .join("lastfm-api-key"))
}
