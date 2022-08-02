use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let config = Config::new(&args);
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    let contents = match fs::read_to_string(&config.filename) {
        Ok(file) => file,
        Err(e) => panic!(
            "could not read file {} , due to errr {}",
            config.filename, e
        ),
    };
    println!("{}", contents);
}
struct Config {
    query: String,
    filename: String,
}
impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();
        Config { query, filename }
    }
}
