#[macro_use]
extern crate clap;
extern crate directories;
extern crate failure;
extern crate indicatif;
extern crate reqwest;
extern crate rusqlite;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod cli;
mod error;
mod exporter;
mod lastfm;
mod paths;
mod db;

fn main() {
    let opts = cli::get_options();

    let db = db::DB::new(&paths::dbpath())
        .expect("failed to create db");
    let lastfm = lastfm::LastFMClient::new(&opts.api_key, &opts.username);
    let exporter = exporter::Exporter::new(&db, &lastfm);

    let to_fetch = exporter.tracks_to_sync().unwrap();
    println!("need to download {} tracks", to_fetch);

    let bar = indicatif::ProgressBar::new(to_fetch);
    bar.set_style(
        indicatif::ProgressStyle::default_bar()
            .progress_chars("=> ")
            .template("{percent:>3}% [{wide_bar}] {eta:5}")
    );

    exporter.sync(|_| { bar.inc(1); })
        .expect("failed to update db");

    bar.finish_with_message("done");
}
