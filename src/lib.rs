use sha1::{Digest, Sha1};
use std::convert::TryInto;
use std::error::Error;
use std::fs::File;
use std::str;

mod binary_search;

pub struct Params {
    pub pass: String,
    pub path: String,
}

impl Params {
    pub fn new(args: &[String]) -> Result<Params, &str> {
        if args.len() < 3 {
            return Err("Too few arguments.\nPlease provide a search term and a file name.");
        }
        let path = args[1].clone();
        let pass = args[2].clone();

        Ok(Params { path, pass })
    }
}

pub fn run(params: Params) -> Result<(), Box<dyn Error>> {
    let hashed_pass = format!("{:X}", Sha1::digest(&params.pass));
    let search_token: [u8; 40] = hashed_pass.as_bytes().try_into().unwrap();

    let file: File = File::open(&params.path).unwrap();
    let file_len = file.metadata().unwrap().len();
    println!(
        "The file is {} bytes long. The search should take aproximately {} iterations.",
        file_len,
        file_len.ilog2()
    );

    match binary_search::search(file, search_token, file_len) {
        Some(index) => println!("Password \"{}\" found at index: {}", &params.pass, index),
        None => println!("Password \"{}\" not found", &params.pass),
    }

    Ok(())
}
