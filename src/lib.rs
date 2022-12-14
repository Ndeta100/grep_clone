use std::error::Error;
use std::process;
use std::{env, fs};
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        //The first value of env::Args is the name of program, so we will skip over that by calling next right away
        let query = match args.next() {
            Some(args) => args,
            None => return Err("Didnt get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file"),
        };
        let case_sensitive = match args.next() {
            Some(arg) => parse_case_sensitivity_arg(arg),
            None => env::var("CASE_INSENSITIVE").is_err(),
        };
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = match fs::read_to_string(&config.filename) {
        Ok(file) => file,
        Err(e) => {
            println!(
                "could not read file {} , due to error {}",
                config.filename, e
            );
            process::exit(1)
        }
    };
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    println!("{}", contents);
    for line in results {
        println!(" \n{}", line);
    }
    Ok(())
}
pub fn parse_case_sensitivity_arg(arg: String) -> bool {
    if arg.to_lowercase() == "insensitive" {
        return false;
    } else {
        return true;
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case_sentitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }
}
#[test]
fn case_insentitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
    assert_eq!(
        vec!["Rust:", "Trust me."],
        search_case_insensitive(query, contents)
    );
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results
}
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
    // let query = query.to_lowercase();
    // let mut results = Vec::new();

    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line);
    //     }
    // }
    // results
}
#[test]
fn case_sensitivity_arg() {
    let case_sensivity_arg = "sensitive".to_string();
    assert_eq!(true, parse_case_sensitivity_arg(case_sensivity_arg));
}
#[test]
fn case_insensitivity_arg() {
    let case_sensivity_arg = "insensitive".to_string();
    assert_eq!(false, parse_case_sensitivity_arg(case_sensivity_arg));
}
#[test]
fn case_insensitivity_arg_invalid() {
    let case_sensivity_arg = "invalid_arg".to_string();
    assert_eq!(true, parse_case_sensitivity_arg(case_sensivity_arg));
}
