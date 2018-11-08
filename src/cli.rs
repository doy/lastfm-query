const _DUMMY_DEPENDENCY: &'static str = include_str!("../Cargo.toml");

pub enum Command {
    Sync {
        username: String,
    }
}

pub fn get_options() -> failure::Fallible<Command> {
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

    let command = match matches.subcommand() {
        ("sync", Some(matches)) => {
            Command::Sync {
                username: matches.value_of("username").unwrap().to_string(),
            }
        },
        (name, Some(_)) => bail!("unknown subcommand: {}", name),
        (_, None) => bail!("no subcommand given"),
    };

    Ok(command)
}
