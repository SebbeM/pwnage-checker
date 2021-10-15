use std::fs;
use std::error::Error;

pub struct Params {
    pub pass: String,
    pub file: String,
}

impl Params {
    pub fn new(args: &[String]) -> Result<Params, &str> {
        if args.len() < 3 {
            return Err("Too few arguments.\nPlease provide a search term and a file name.")
        }
        let pass = args[1].clone();
        let file = args[2].clone();

        Ok(Params { pass, file })
    }
}

pub fn run(params: Params) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(params.file)?;

    println!("With text:\n{}", contents);

    Ok(())
}
