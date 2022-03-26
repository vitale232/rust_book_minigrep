use std::env;
use std::process;

use rust_book_minigrep;
use rust_book_minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("A problem occurred while parsing the arguments: {}", err);

        process::exit(1);
    });

    if let Err(e) = rust_book_minigrep::run(config) {
        eprintln!("An application error occurred! {}", e);

        process::exit(1);
    }
}
