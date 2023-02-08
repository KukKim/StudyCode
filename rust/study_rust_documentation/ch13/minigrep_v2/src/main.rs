use std::env;
use std::process;

use minigrep_v2::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();

    // let config = Config::new(&args).unwrap_or_else(|err| {
    //     eprintln!("parameter error: {}", err);
    //     process::exit(1);
    // });

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("parameter error: {}", err);
        process::exit(1);
    });

    println!("query: {}", config.query);
    println!("filename: {}", config.filename);

    if let Err(e) = minigrep_v2::run(config) {
        eprintln!("Application Error: {}", e);

        process::exit(1);
    }
}
