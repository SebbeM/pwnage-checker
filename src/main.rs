use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let pass = &args[1];
    let file = &args[2];

    println!("Searching for {}", pass);
    println!("In file {}", file);

    let contents = fs::read_to_string(file)
        .expect("Something went wrong when reading the file");

    println!("With text:\n{}", contents);
}
