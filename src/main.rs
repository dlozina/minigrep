use std::env;
use std::process;

use minigrep::Config;

// Commands to test minigrep
//cargo run -- to poem.txt
// IGNORE_CASE=1 cargo run -- to poem.txt
// IGNORE_CASE=1 cargo run -- to poem.txt > output.txt
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem with parsing arguments: {err}");
        process::exit(1)
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1)
    }
}
