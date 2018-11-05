extern crate directories;
extern crate failure;
extern crate rusqlite;

mod error;
mod paths;
mod db;

fn main() {
    let db = db::DB::new(&paths::dbpath())
        .expect("failed to create db");
}
