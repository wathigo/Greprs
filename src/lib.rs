use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub struct Config {
    search_query: String,
    file_name: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments \nRun with <search-query> <file-name>");
        }
        let search_query = args[1].clone();
        let file_name = args[2].clone();

        Ok(Config { search_query, file_name })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Search query `{}`", config.search_query);
    println!("In file `{}`", config.file_name);

    let mut f = File::open(&config.file_name)?;

    let mut content = String::new();
    f.read_to_string(&mut content)?;

    println!("File {} Contains \n{}",config.file_name, content);
    Ok(())
}