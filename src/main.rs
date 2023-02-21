use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = minigrep::Config::build(&args).unwrap_or_else(|err| {
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        process::exit(1);
    };
}
