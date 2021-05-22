// std::env imports standard library for interacting with environment
use std::env;
// std::process imports a standard library for working with processes
use std::process;

use minigrep::Config;

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
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    };
}
