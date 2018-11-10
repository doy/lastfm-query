mod sql;
mod sync;
mod recommend;

const _DUMMY_DEPENDENCY: &'static str = include_str!("../../Cargo.toml");

trait Command {
    fn run(&self) -> failure::Fallible<()>;
}

pub fn run() -> failure::Fallible<()> {
    get_command()?.run()
}

fn get_command() -> failure::Fallible<Box<Command>> {
    let matches = app_from_crate!()
        .subcommand(sync::subcommand())
        .subcommand(sql::subcommand())
        .subcommand(recommend::subcommand())
        .get_matches();

    let command: Box<Command> = match matches.subcommand() {
        ("sync", Some(matches)) => Box::new(sync::Command::new(matches)),
        ("sql", Some(matches)) => Box::new(sql::Command::new(matches)),
        ("recommend", Some(matches)) => Box::new(recommend::Command::new(matches)?),

        (name, Some(_)) => bail!("unknown subcommand: {}", name),
        (_, None) => bail!("no subcommand given"),
    };

    Ok(command)
}
