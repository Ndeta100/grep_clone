use std::env;
use std::fs;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    run(config);
}
struct Config {
    query: String,
    filename: String,
}
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}
fn run(config: Config) {
    let contents = match fs::read_to_string(&config.filename) {
        Ok(file) => file,
        Err(e) => {
            println!(
                "could not read file {} , due to errr {}",
                config.filename, e
            );
            process::exit(1)
        }
    };
    println!("{}", contents);
}
