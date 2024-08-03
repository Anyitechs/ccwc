use std::env;
use std::process;
use ccwc::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("An error occured: {err}");
        process::exit(1);
    });

    if let Err(e) = ccwc::run(config) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
