use clap;

const _DUMMY_DEPENDENCY: &'static str = include_str!("../Cargo.toml");

pub struct Options {
    pub username: String,
    pub api_key: String,
}

pub fn get_options() -> Options {
    let matches = app_from_crate!()
        .arg(
            clap::Arg::with_name("username")
                .required(true)
                .help("last.fm username to fetch tracks for")
        )
        .arg(
            clap::Arg::with_name("api_key")
                .required(true)
                .help("last.fm api key")
        )
        .get_matches();

    Options {
        username: matches.value_of("username").unwrap().to_string(),
        api_key: matches.value_of("api_key").unwrap().to_string(),
    }
}
