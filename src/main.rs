use std::env;
use std::process;

use pwnage_checker::Params;

fn main() {
    let args: Vec<String> = env::args().collect();

    let params = Params::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);

        process::exit(1);
    });

    println!("Searching for {}", params.pass);
    println!("In file {}", params.path);

    if let Err(e) = pwnage_checker::run(params) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
