// brings Error into use; use for basic expectations in error values
use std::error::Error;
// std::fs imports standard library for interacting with files
use std::fs;

// Box trait object; will return a type that implements the Error trait
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
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

pub struct Config {
    pub query: String,
    pub filename: String,
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

        Ok( Config { query, filename } )
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
}
