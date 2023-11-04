use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::env;

pub struct Config {
    search_query: String,
    file_name: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments \nRun with <search-query> <file-name>");
        }
        let search_query = args[1].clone();
        let file_name = args[2].clone();
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();

        Ok(Config { search_query, file_name, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Search query `{}`", config.search_query);
    println!("In file `{}`", config.file_name);

    let mut f = File::open(&config.file_name)?;

    let mut content = String::new();
    f.read_to_string(&mut content)?;

    let results = if config.case_sensitive {
        search(&config.search_query, &content)
    } else {
        search_case_insensitive(&config.search_query, &content)   
    };

    for line in results {
        println!("{}", line);
    }
    
    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
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
        fn case_sensitive() {
            let query = "duct";
            let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";


            assert_eq!(
                vec!["safe, fast, productive."],
                search(query, content)
            );
        }

        #[test]
        fn case_insensitive() {
            let query = "rUsT";
            let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
            assert_eq!(
                vec!["Rust:", "Trust me."],
                search_case_insensitive(query, contents)
            );
        }
    }

