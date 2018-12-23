mod cmd;
mod db;
mod lastfm;
mod util;

fn main() {
    match cmd::run() {
        Ok(_) => {}
        Err(e) => {
            let name = util::program_name().unwrap_or_else(|e| {
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
