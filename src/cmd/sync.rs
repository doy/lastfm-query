use db;
use lastfm;
use paths;

pub fn run(username: &str) -> failure::Fallible<()> {
    let db = db::DB::new(&paths::db_path()?)?;
    let lastfm = lastfm::LastFMClient::new(username)?;

    let from = db.most_recent_timestamp()?.map(|x| x + 1);
    let to_fetch = lastfm.track_count(from)?;

    if to_fetch > 0 {
        let bar = indicatif::ProgressBar::new(to_fetch);
        bar.set_style(
            indicatif::ProgressStyle::default_bar()
                .progress_chars("=> ")
                .template("Downloading {pos}/{len} tracks...\n{percent:>3}% [{wide_bar}] {eta:5}")
        );

        db.insert_tracks(bar.wrap_iter(lastfm.tracks(from)))?;

        bar.finish_with_message("done");
    }

    Ok(())
}
