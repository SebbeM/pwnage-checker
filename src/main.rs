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

    let result = if let Some(_path) = &params.path {
        pwnage_checker::file_search(params)
    } else {
        pwnage_checker::range_search(params)
    };

    if let Err(e) = result {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
