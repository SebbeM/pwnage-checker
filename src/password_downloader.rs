use anyhow::{Error, Ok};
use reqwest::blocking::Response;
use std::{fs::File, io::Write};

static RANGE_ENDPOINT: &str = "https://api.pwnedpasswords.com/range/";

pub fn download_passwords(path: &str) -> Result<File, Error> {
    let mut file = File::create(path)?;
    let size = 0;
    for range in 0x00000..0xfffff {
        let buf = download_range(range).bytes()?;
        let written = file.write(&buf)?;
        file.write(b"\r\n")?;
        println!("{} bytes written, {} in total", written, size);
    }
    Ok(file)
}

pub fn download_range(range: u32) -> Response {
    let target = String::from(RANGE_ENDPOINT) + &format!("{:05X}", range);
    println!("Downloading target: {}", target);
    let response: Response = reqwest::blocking::get(target).expect("Failed to fetch");
    return response;
}
