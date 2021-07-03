// brings Error into use; use for basic expectations in error values
use std::error::Error;
// std::fs imports standard library for interacting with files
use std::fs;
// std::env imports functions for working with env variables
use std::env;

// Box trait object; will return a type that implements the Error trait
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    // Returns an 'Ok' value in case of success
    Ok(())
}

// 'a in the args here is a lifetime annotation; ensures that the value persists long
// enough to be used without going out of scope
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new(); // create a mutable vector to store results
    // lines; method for iterating through string with line breaks (\n or \r\n)
    // returns an iterator
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    // implicit return
    results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        // now takes a reference &query because to_lowercase returns a new str
        // since the original str slice wouldn't contain the case insensitive query str
        if line.to_lowercase().contains(&query) { 
            results.push(line);
        }
    }

    results
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

// implements function that takes collection of strings as args; returns a Result with a Config
// instance on success or a String on failure
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        // Throws error if args is of insufficient length
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone(); // clones values for storage in config struct
        let filename = args[2].clone(); // less efficient, but easier to do

        // returns true if it errors when trying to find the env var; if it errors,
        // it means that it's unset and therefore false
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive
        })
    }
}

// Rust permits in-file tests 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, produtive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
