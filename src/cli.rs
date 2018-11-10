const _DUMMY_DEPENDENCY: &'static str = include_str!("../Cargo.toml");

pub enum Command {
    Sync {
        username: String,
    },
    SQL {
        query: String,
        tsv: bool,
    },
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
        .subcommand(
            clap::SubCommand::with_name("sql")
                .about("Run a query against the local database")
                .arg(
                    clap::Arg::with_name("query")
                        .required(true)
                        .help("query to run")
                )
                .arg(
                    clap::Arg::with_name("tsv")
                        .long("tsv")
                        .help("format output as tsv")
                )
        )
        .get_matches();

    let command = match matches.subcommand() {
        ("sync", Some(matches)) => {
            Command::Sync {
                username: matches.value_of("username").unwrap().to_string(),
            }
        },
        ("sql", Some(matches)) => {
            Command::SQL {
                query: matches.value_of("query").unwrap().to_string(),
                tsv: matches.is_present("tsv"),
            }
        },
        (name, Some(_)) => bail!("unknown subcommand: {}", name),
        (_, None) => bail!("no subcommand given"),
    };

    Ok(command)
}
