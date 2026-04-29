use std::env;
use std::process;

use pwnage_checker::Params;

fn main() {
    let args: Vec<String> = env::args().collect();

    let params = if args.len() == 1 {
        match rpassword::prompt_password("Password: ") {
            Ok(pass) => Params { pass, path: None },
            Err(err) => {
                println!("Failed to read password: {}", err);
                process::exit(1);
            }
        }
    } else {
        Params::new(&args).unwrap_or_else(|err| {
            println!("Problem parsing arguments: {}", err);
            process::exit(1);
        })
    };

    let result = if let Some(path) = &params.path {
        println!("Searching for {} in file {}", params.pass, path);
        pwnage_checker::file_search(params)
    } else {
        println!("Searching for {} via HIBP range API", params.pass);
        pwnage_checker::range_search(params)
    };

    if let Err(e) = result {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
