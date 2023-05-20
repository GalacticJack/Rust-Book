use ch12_minigrep::Config;
use std::{env, println, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // Effectively the same as .unwrap_or_else
    if let Err(e) = ch12_minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
