use std::env;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let search_query = &arguments[1];
    let file_name = &arguments[2];

    println!("Search query `{}`", search_query);
    println!("In file `{}`", file_name);
}
