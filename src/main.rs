use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (password, filename) = parse(&args);

    println!("Searching for {}", password);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong when reading the file");

    println!("With text:\n{}", contents);
}

fn parse(args: &[String]) -> (&str, &str) {
    let password = &args[1];
    let filename = &args[2];

    (password, filename)
}
