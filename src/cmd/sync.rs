use cli;
use db;
use lastfm;
use paths;

use failure;

pub fn run(opts: &cli::Options) -> failure::Fallible<()> {
    let db = db::DB::new(&paths::dbpath())?;
    let lastfm = lastfm::LastFMClient::new(
        opts.api_key.as_ref().unwrap(),
        opts.username.as_ref().unwrap()
    );

    let ts = db.most_recent_timestamp()?;
    let to_fetch = lastfm.track_count(ts.map(|x| x + 1))?;

    let bar = indicatif::ProgressBar::new(to_fetch);
    bar.set_style(
        indicatif::ProgressStyle::default_bar()
            .progress_chars("=> ")
            .template("Downloading {pos}/{len} tracks...\n{percent:>3}% [{wide_bar}] {eta:5}")
    );

    db.insert_tracks(bar.wrap_iter(lastfm.tracks(ts.map(|x| x + 1))))?;

    bar.finish_with_message("done");

    Ok(())
}
