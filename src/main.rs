use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // collect is one function you do often need to
    // annotate because Rust isn't able to infer the kind
    // of collection you want
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
