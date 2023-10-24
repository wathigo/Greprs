use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::error::Error;

struct Config {
    search_query: String,
    file_name: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments \nRun with <search-query> <file-name>");
        }
        let search_query = args[1].clone();
        let file_name = args[2].clone();

        Ok(Config { search_query, file_name })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Search query `{}`", config.search_query);
    println!("In file `{}`", config.file_name);

    let mut f = File::open(&config.file_name)?;

    let mut content = String::new();
    f.read_to_string(&mut content)?;

    println!("File {} Contains \n{}",config.file_name, content);
    Ok(())
}

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let config = Config::new(&arguments).unwrap_or_else(|err| {
        println!("Program stopped with error: \n{}", err);
        process::exit(1);
    });
    
    if let Err(e) = run(config) {
        println!("Application stopped!\n{}", e);
        process::exit(1);
    };
}
