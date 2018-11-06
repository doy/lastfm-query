extern crate directories;
extern crate failure;
extern crate reqwest;
extern crate rusqlite;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod error;
mod lastfm;
mod paths;
mod db;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 3 {
        panic!("usage: {} USERNAME API_KEY", args[0]);
    }
    let username = &args[1];
    let api_key = &args[2];

    let db = db::DB::new(&paths::dbpath())
        .expect("failed to create db");
    let lastfm = lastfm::LastFMClient::new(api_key, username);

    println!("{}", lastfm.track_count().unwrap());

    for track in lastfm.tracks().take(10) {
        println!("{}", track.name);
    }
}
