use cli;
use db;
use exporter;
use lastfm;
use paths;

use failure;

pub fn run(opts: &cli::Options) -> failure::Fallible<()> {
    let db = db::DB::new(&paths::dbpath())?;
    let lastfm = lastfm::LastFMClient::new(
        opts.api_key.as_ref().unwrap(),
        opts.username.as_ref().unwrap()
    );

    let exporter = exporter::Exporter::new(&db, &lastfm);

    let to_fetch = exporter.tracks_to_sync()?;
    let bar = indicatif::ProgressBar::new(to_fetch);
    bar.set_style(
        indicatif::ProgressStyle::default_bar()
            .progress_chars("=> ")
            .template("Downloading {pos}/{len} tracks...\n{percent:>3}% [{wide_bar}] {eta:5}")
    );

    exporter.sync(|_| { bar.inc(1); })?;

    bar.finish_with_message("done");

    Ok(())
}
