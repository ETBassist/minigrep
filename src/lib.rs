// brings Error into use; use for basic expectations in error values
use std::error::Error;
// std::fs imports standard library for interacting with files
use std::fs;

// Box trait object; will return a type that implements the Error trait
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("You're searching for {}", config.query);
    println!("In {}", config.filename);

    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n\n{}", contents);

    // Returns an 'Ok' value in case of success
    Ok(())
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
