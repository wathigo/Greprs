use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let search_query = &arguments[1];
    let file_name = &arguments[2];
    println!("Search query `{}`", search_query);
    println!("In file `{}`", file_name);

    let mut f = File::open(file_name).expect("Couldn't open file!");

    let mut content = String::new();
    f.read_to_string(&mut content).expect("Couldn't read file contents!");

    println!("File {} contains \n{}",file_name, content);
}
