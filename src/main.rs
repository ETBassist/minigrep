// std::env imports standard library for interacting with environment
use std::env;
// std::fs imports standard library for interacting with files
use std::fs;

fn main() {
    // env::args(); returns an iterator of command line args
    let args: Vec<String> = env::args().collect(); // .collect() turns iterator into a collection

    // index 0 is the file from which this is run (courtesy of std::env::args)
    // will panic if no args given from cli
    let query = &args[1]; // reference
    let filename = &args[2];

    println!("You're searching for {}", query);
    println!("In {}", filename);

    // read in file content to a string
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong with reading in the file");

    println!("With text:\n\n{}", contents)
}
