use crate::db;
use crate::lastfm;
use crate::util;

use clap;

pub struct Command {
    username: String,
}

pub fn subcommand<'a, 'b>() -> clap::App<'a, 'b> {
    clap::SubCommand::with_name("sync")
        .about("Updates the local copy of track data from last.fm")
        .arg(
            clap::Arg::with_name("username")
                .required(true)
                .help("last.fm username to fetch tracks for"),
        )
}

impl Command {
    pub fn new<'a>(matches: &clap::ArgMatches<'a>) -> Command {
        Command {
            username: matches.value_of("username").unwrap().to_string(),
        }
    }
}

impl super::Command for Command {
    fn run(&self) -> failure::Fallible<()> {
        let db = db::DB::new(&util::db_path()?)?;
        let lastfm = lastfm::LastFMClient::new(&self.username)?;

        let from = db.most_recent_timestamp()?.map(|x| x + 1);
        let to_fetch = lastfm.track_count(from)?;

        if to_fetch > 0 {
            let pbar = indicatif::ProgressBar::new(to_fetch);
            pbar.set_style(
                indicatif::ProgressStyle::default_bar()
                    .progress_chars("=> ")
                    .template(
                        "Downloading {pos}/{len} tracks...\n\
                         {percent:>3}% [{wide_bar}] {eta:5}",
                    ),
            );

            db.insert_tracks(pbar.wrap_iter(lastfm.tracks(from)))?;

            pbar.finish_with_message("done");
        }

        Ok(())
    }
}
