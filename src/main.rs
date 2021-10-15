use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let params = Params::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", params.pass);
    println!("In file {}", params.file);

    run(params);
}

fn run(params: Params) {
    let contents = fs::read_to_string(params.file)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

struct Params {
    pass: String,
    file: String,
}

impl Params {
    fn new(args: &[String]) -> Result<Params, &str> {
        if args.len() < 3 {
            return Err("Too few arguments.\nPlease provide a search term and a file name.")
        }
        let pass = args[1].clone();
        let file = args[2].clone();

        Ok(Params { pass, file })
    }
}
