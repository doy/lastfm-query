pub mod sql;
pub mod sync;

const _DUMMY_DEPENDENCY: &'static str = include_str!("../../Cargo.toml");

pub trait Command {
    fn run(&self) -> failure::Fallible<()>;
}

pub fn run() -> failure::Fallible<()> {
    get_command()?.run()
}

fn get_command() -> failure::Fallible<Box<Command>> {
    let matches = app_from_crate!()
        .subcommand(sync::subcommand())
        .subcommand(sql::subcommand())
        .get_matches();

    let command: Box<Command> = match matches.subcommand() {
        ("sync", Some(matches)) => Box::new(sync::Command::new(matches)),
        ("sql", Some(matches)) => Box::new(sql::Command::new(matches)),

        (name, Some(_)) => bail!("unknown subcommand: {}", name),
        (_, None) => bail!("no subcommand given"),
    };

    Ok(command)
}
