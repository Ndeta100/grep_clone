use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let (query, filename) = parse_config(&args);
    println!("Searching for {}", query);
    println!("In file {}", filename);
    let contents = match fs::read_to_string(filename) {
        Ok(file) => file,
        Err(e) => panic!("could not read file {} , due to errr {}", filename, e),
    };
    println!("{}", contents);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];
    (query, filename)
}
