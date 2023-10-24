extern crate greprs;

use std::env;
use std::process;

use greprs::Config;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let config = Config::new(&arguments).unwrap_or_else(|err| {
        println!("Program stopped with error: \n{}", err);
        process::exit(1);
    });
    
    if let Err(e) = greprs::run(config) {
        println!("Application stopped!\n{}", e);
        process::exit(1);
    };
}
