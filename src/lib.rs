use anyhow::{Error, Ok};
use sha1::{Digest, Sha1};
use std::convert::TryInto;
use std::fs::File;
use std::path::Path;

mod binary_search;
mod password_downloader;

pub struct Params {
    pub pass: String,
    pub path: Option<String>,
}

impl Params {
    pub fn new(args: &[String]) -> Result<Params, Error> {
        if args.len() == 3 {
            return Ok(Params {
                path: Some(args[1].clone()),
                pass: args[2].clone(),
            });
        } else if args.len() == 2 {
            return Ok(Params {
                pass: args[1].clone(),
                path: None,
            });
        }

        return Err(Error::msg(
            "Too few arguments.\nPlease provide a password, and optionally a hash file.",
        ));
    }
}

pub fn file_search(params: Params) -> Result<(), Error> {
    let hashed_pass = format!("{:X}", Sha1::digest(&params.pass));
    let search_token: [u8; 40] = hashed_pass.as_bytes().try_into()?;

    let path_str = params.path.ok_or_else(|| Error::msg("No file path provided"))?;
    let file: File;
    let path = Path::new(&path_str);
    if Path::exists(path) {
        file = File::open(path)?;
    } else {
        println!("File {} not found. Downloading from HIBP API...", &path_str);
        file = password_downloader::download_passwords(&path_str)?;
    }
    let file_len = file.metadata()?.len();
    println!(
        "The file is {} bytes long. The search should take aproximately {} iterations.",
        file_len,
        file_len.ilog2()
    );

    match binary_search::file_search(file, search_token, file_len) {
        Some(index) => println!("Hash {} found at index: {}", hashed_pass, index),
        None => println!("Hash {} not found", hashed_pass),
    }

    Ok(())
}

pub fn range_search(params: Params) -> Result<(), Error> {
    let hashed_pass = format!("{:X}", Sha1::digest(&params.pass));
    let (range, rest) = hashed_pass.split_at(5);
    let suffix_token: [u8; 35] = rest.as_bytes().try_into()?;
    let response = password_downloader::download_range(u32::from_str_radix(range, 16)?);
    let arr: Vec<u8> = response.bytes()?.to_vec();
    let end = arr.len() as u64;

    match binary_search::range_search(&arr, suffix_token, end) {
        Some(index) => println!("Hash {} found at index: {}", hashed_pass, index),
        None => println!("Hash {} not found", hashed_pass),
    }

    Ok(())
}
