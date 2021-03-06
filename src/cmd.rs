mod recommend;
mod sql;
mod sync;

use clap::{
    app_from_crate, crate_authors, crate_description, crate_name,
    crate_version,
};
use failure::bail;

const _DUMMY_DEPENDENCY: &str = include_str!("../Cargo.toml");

trait Command {
    fn run(&self) -> failure::Fallible<()>;
}

pub fn run() -> failure::Fallible<()> {
    get_command()?.run()
}

fn get_command() -> failure::Fallible<Box<Command>> {
    let subcommands = vec![
        sync::subcommand(),
        sql::subcommand(),
        recommend::subcommand(),
    ];
    let mut app = app_from_crate!().subcommands(
        subcommands
            .into_iter()
            .map(|s| s.setting(clap::AppSettings::DisableVersion)),
    );
    let matches = app.clone().get_matches();

    let command: Box<Command> = match matches.subcommand() {
        ("sync", Some(matches)) => Box::new(sync::Command::new(matches)),
        ("sql", Some(matches)) => Box::new(sql::Command::new(matches)),
        ("recommend", Some(matches)) => {
            Box::new(recommend::Command::new(matches)?)
        }

        (name, Some(_)) => bail!("unknown subcommand: {}", name),
        (_, None) => {
            let mut stderr = std::io::stderr();
            app.write_long_help(&mut stderr)?;
            eprintln!("");
            bail!("no subcommand given")
        }
    };

    Ok(command)
}
