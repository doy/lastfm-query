const _DUMMY_DEPENDENCY: &'static str = include_str!("../Cargo.toml");

pub enum Command {
    Sync,
}

pub struct Options {
    pub command: Command,
    pub username: Option<String>,
}

pub fn get_options() -> failure::Fallible<Options> {
    let matches = app_from_crate!()
        .subcommand(
            clap::SubCommand::with_name("sync")
                .about("Updates the local copy of track data from last.fm")
                .arg(
                    clap::Arg::with_name("username")
                        .required(true)
                        .help("last.fm username to fetch tracks for")
                )
        )
        .get_matches();

    let (command, sub_matches) = match matches.subcommand() {
        ("sync", Some(matches)) => (Command::Sync, matches),
        (name, Some(_)) => bail!("unknown subcommand: {}", name),
        (_, None) => bail!("no subcommand given"),
    };

    Ok(Options {
        command: command,
        username: sub_matches.value_of("username").map(|s| s.to_string()),
    })
}
