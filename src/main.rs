use grep_cli::Config;
use std::env;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    if let Err(e) = grep_cli::run(config) {
        eprintln!("application error {}", e);
        process::exit(1);
    }
}
