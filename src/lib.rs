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

    for line in search(&config.search_query, &content) {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results =  Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn one_result() {
            let query = "duct";
            let content = "\
Rust:
safe, fast, productive.
Pick three.";


            assert_eq!(
                vec!["safe, fast, productive."],
                search(query, content)
            );
        }
    }

