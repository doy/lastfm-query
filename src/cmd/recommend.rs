use db;
use util;

use clap;

pub struct Command {
    count: u64,
    random: bool,
    album: bool,
    include: db::TimeWindow,
    exclude: db::TimeWindow,

    db: db::DB,
}

pub fn subcommand<'a, 'b>() -> clap::App<'a, 'b> {
    clap::SubCommand::with_name("recommend")
        .about("Recommends an artist or album to listen to")
        .arg(
            clap::Arg::with_name("count")
                .default_value("20")
                .help("number of results to return")
        )
        .arg(
            clap::Arg::with_name("random")
                .long("random")
                .help("picks randomly instead of by weight")
        )
        .arg(
            clap::Arg::with_name("album")
                .long("album")
                .help("also choose a random album by the chosen artists")
        )
        .arg(
            clap::Arg::with_name("include")
                .long("include")
                .default_value("yearly")
                .possible_values(&["all", "yearly", "monthly", "weekly"])
        )
        .arg(
            clap::Arg::with_name("exclude")
                .long("exclude")
                .default_value("weekly")
                .possible_values(&["all", "yearly", "monthly", "weekly", "none"])
        )
}

impl Command {
    pub fn new<'a>(matches: &clap::ArgMatches<'a>) -> failure::Fallible<Command> {
        Ok(Command {
            count: matches.value_of("count").unwrap().parse()?,
            random: matches.is_present("random"),
            album: matches.is_present("album"),
            include: db::parse_timewindow(matches.value_of("include").unwrap()),
            exclude: db::parse_timewindow(matches.value_of("exclude").unwrap()),

            db: db::DB::new(&util::db_path()?)?,
        })
    }
}

impl super::Command for Command {
    fn run(&self) -> failure::Fallible<()> {
        let mut artists = self.db.recommend_artists(
            self.count,
            self.random,
            self.include,
            self.exclude
        )?;
        if self.album {
            artists = artists.iter().map(|artist| {
                Ok(format!(
                    "{} - {}",
                    artist,
                    self.db.recommend_album(
                        &artist,
                        self.random,
                        self.include,
                        self.exclude
                    )?
                ))
            }).collect::<failure::Fallible<Vec<String>>>()?;
        }
        for line in artists {
            println!("{}", line);
        }
        Ok(())
    }
}
