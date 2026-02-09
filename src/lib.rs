use sha1::{Digest, Sha1};
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::error::Error;
use std::f32::consts::LOG10_2;
use std::fs::File;
use std::iter;
use std::os::unix::fs::FileExt;
use std::u64;

pub struct Params {
    pub pass: String,
    pub path: String,
}

impl Params {
    pub fn new(args: &[String]) -> Result<Params, &str> {
        if args.len() < 3 {
            return Err("Too few arguments.\nPlease provide a search term and a file name.");
        }
        let pass = args[1].clone();
        let path = args[2].clone();

        Ok(Params { pass, path })
    }
}

pub fn run(params: Params) -> Result<(), Box<dyn Error>> {
    let hashed_pass = format!("{:X}", Sha1::digest(params.pass.clone()));
    let search_token: [u8; 40] = hashed_pass.as_bytes().try_into().unwrap();

    let file: File = File::open(params.path.clone()).unwrap();

    let len = file.metadata().unwrap().len();
    println!(
        "The file is {} bytes long. The search should take aproximately {} iterations.",
        len,
        len.ilog2()
    );

    match binary_search(file, search_token, len) {
        Some(index) => println!(
            "Binary search found \"{}\" at index: {}",
            params.pass, index
        ),
        None => println!("Password \"{}\" not found", params.pass),
    }

    Ok(())
}

fn binary_search(file: File, token: [u8; 40], end: u64) -> Option<usize> {
    let mut low: u64 = 0;
    let mut high: u64 = u64::try_from(end - 1).unwrap();
    let mut iterations: u32 = 0;
    let buf: &mut [u8; 40] = &mut [0; 40];

    while low <= high {
        iterations += 1;
        let mid: u64 = (low + high) / 2;
        eprintln!("Middle is now at {}", mid);

        let mut res = file.read_exact_at(buf, mid);
        let newline_index = buf.iter().position(|&r| r == b':');
        if newline_index.is_some() {
            let new_line: u64 = newline_index.unwrap().try_into().unwrap();
            let offset = mid + new_line - 40;
            res = file.read_exact_at(buf, offset);
        };

        match res {
            Err(e) => {
                println!(
                    "Failed to read {} bytes at position {} due to error {}.",
                    buf.len(),
                    mid,
                    e
                );
                return None;
            }
            Ok(_) => match token.cmp(buf) {
                Ordering::Equal => {
                    println!("Found token {:?} after {} iterations", token, iterations);
                    return Some(usize::try_from(mid).unwrap());
                }
                Ordering::Less => {
                    println!("{:?} is greater than {:?}", token, buf);
                    high = mid - 1
                }
                Ordering::Greater => {
                    println!("{:?} is less than {:?}", token, buf);
                    low = mid + 1
                }
            },
        }
    }
    None
}
