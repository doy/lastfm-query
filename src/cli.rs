use cmd;

const _DUMMY_DEPENDENCY: &'static str = include_str!("../Cargo.toml");

enum Command {
    Sync(cmd::sync::Options),
    SQL(cmd::sql::Options),
}

pub fn run() -> failure::Fallible<()> {
    match get_options()? {
        Command::Sync(options) => cmd::sync::run(&options),
        Command::SQL(options) => cmd::sql::run(&options),
    }
}

fn get_options() -> failure::Fallible<Command> {
    let matches = app_from_crate!()
        .subcommand(cmd::sync::subcommand())
        .subcommand(cmd::sql::subcommand())
        .get_matches();

    let command = match matches.subcommand() {
        ("sync", Some(matches)) => Command::Sync(cmd::sync::options(matches)),
        ("sql", Some(matches)) => Command::SQL(cmd::sql::options(matches)),

        (name, Some(_)) => bail!("unknown subcommand: {}", name),
        (_, None) => bail!("no subcommand given"),
    };

    Ok(command)
}
