// std::env imports standard library for interacting with environment
use std::env;
// std::fs imports standard library for interacting with files
use std::fs;
// std::process imports a standard library for working with processes
use std::process;
// brings Error into use; use for basic expectations in error values
use std::error::Error;

fn main() {
    // env::args(); returns an iterator of command line args
    let args: Vec<String> = env::args().collect(); // .collect() turns iterator into a collection

    // index 0 is the file from which this is run (courtesy of std::env::args)
    // will panic if no args given from cli
    // .unwrap_or_else is defined on Result<T, E>; returns value for Ok if present, or else refers
    // to the closure (which can use the value for Err)
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("You're searching for {}", config.query);
    println!("In {}", config.filename);

    // read in file content to a string
    // don't need to worry about using an unwrap because the return of this function isn't relevant
    // only that the code within executes
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    };
}

// Box trait object; will return a type that implements the Error trait
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("You're searching for {}", config.query);
    println!("In {}", config.filename);

    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n\n{}", contents);

    // Returns an 'Ok' value in case of success
    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

// implements function that takes collection of strings as args; returns a Result with a Config
// instance on success or a String on failure
impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        // Throws error if args is of insufficient length
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone(); // clones values for storage in config struct
        let filename = args[2].clone(); // less efficient, but easier to do

        Ok( Config { query, filename } )
    }
}
