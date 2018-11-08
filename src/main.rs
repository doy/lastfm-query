#[macro_use]
extern crate clap;
extern crate directories;
#[macro_use]
extern crate failure;
extern crate indicatif;
extern crate reqwest;
extern crate rusqlite;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod cli;
mod exporter;
mod lastfm;
mod paths;
mod db;

fn run(opts: cli::Options) -> failure::Fallible<()> {
    let db = db::DB::new(&paths::dbpath())?;
    let lastfm = lastfm::LastFMClient::new(&opts.api_key, &opts.username);
    let exporter = exporter::Exporter::new(&db, &lastfm);

    let to_fetch = exporter.tracks_to_sync()?;
    println!("need to download {} tracks", to_fetch);

    let bar = indicatif::ProgressBar::new(to_fetch);
    bar.set_style(
        indicatif::ProgressStyle::default_bar()
            .progress_chars("=> ")
            .template("{percent:>3}% [{wide_bar}] {eta:5}")
    );

    exporter.sync(|_| { bar.inc(1); })?;

    bar.finish_with_message("done");

    Ok(())
}

fn program_name() -> failure::Fallible<String> {
    let program = std::env::args()
        .next()
        .ok_or_else(|| format_err!("no program name found"))?;
    let path = std::path::Path::new(&program);
    let filename = path.file_name()
        .ok_or_else(|| format_err!("invalid filename found"))?
        .to_string_lossy()
        .to_string();
    Ok(filename)
}

fn main() {
    let opts = cli::get_options();
    match run(opts) {
        Ok(_) => {},
        Err(e) => {
            let name = program_name()
                .unwrap_or_else(|e| {
                    eprintln!("{}", e);
                    "?".to_string()
                });
            let cause = e
                .iter_chain()
                .fold(String::new(), |acc, x| acc + ": " + &format!("{}", x));
            eprintln!("{}{}", name, cause);
            std::process::exit(1);
        }
    }
}
