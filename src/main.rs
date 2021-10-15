use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let params = parse(&args);

    println!("Searching for {}", params.pass);
    println!("In file {}", params.file);

    let contents = fs::read_to_string(params.file)
        .expect("Something went wrong when reading the file");

    println!("With text:\n{}", contents);
}

struct Params {
    pass: String,
    file: String,
}

fn parse(args: &[String]) -> Params {
    let pass = args[1].clone();
    let file = args[2].clone();

    Params { pass, file }
}
