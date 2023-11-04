extern crate greprs;

use std::env;
use std::process;
use std::io::Write;

use greprs::Config;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let mut stderr = std::io::stderr();

    let config = Config::new(&arguments).unwrap_or_else(|err| {
        writeln!(stderr, "Program stopped with error: \n{}", err)
        .expect("Cannot write to standard error!");
        process::exit(1);
    });
    
    if let Err(e) = greprs::run(config) {
        println!("Application stopped!\n{}", e);
        process::exit(1);
    };
}
