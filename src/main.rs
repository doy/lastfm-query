#[macro_use]
extern crate clap;
extern crate directories;
#[macro_use]
extern crate failure;
extern crate indicatif;
extern crate reqwest;
extern crate rpassword;
extern crate rusqlite;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod cli;
mod cmd;
mod lastfm;
mod paths;
mod db;

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
    match cli::run() {
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
