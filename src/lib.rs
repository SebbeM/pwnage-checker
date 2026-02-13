use anyhow::{Error, Ok};
use sha1::{Digest, Sha1};
use std::convert::TryInto;
use std::fs::File;
use std::path::Path;
use std::str;

mod binary_search;
mod password_downloader;

static DEFAULT_PATH: &str = "passwords.txt";

pub struct Params {
    pub pass: String,
    pub path: String,
}

impl Params {
    pub fn new(args: &[String]) -> Result<Params, Error> {
        if args.len() == 3 {
            return Ok(Params {
                path: args[1].clone(),
                pass: args[2].clone(),
            });
        } else if args.len() == 2 {
            return Ok(Params {
                pass: args[1].clone(),
                path: String::from(DEFAULT_PATH),
            });
        }

        return Err(Error::msg(
            "Too few arguments.\nPlease provide a search term and a file name.",
        ));
    }
}

pub fn file_search(params: Params) -> Result<(), Error> {
    let hashed_pass = format!("{:X}", Sha1::digest(&params.pass));
    let search_token: [u8; 40] = hashed_pass.as_bytes().try_into()?;

    let file: File;
    let path = Path::new(&params.path);
    if Path::exists(path) {
        file = File::open(path)?;
    } else {
        println!(
            "File {} not found. Downloading from HIBP API...",
            &params.path
        );
        file = password_downloader::download_passwords(&params.path)?;
    }
    let file_len = file.metadata()?.len();
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
