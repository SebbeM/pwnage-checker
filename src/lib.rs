use std::fs::File;
use std::error::Error;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Seek;
use std::io::SeekFrom;
use std::cmp::Ordering;
use std::convert::TryFrom;

pub struct Params {
    pub pass: String,
    pub path: String,
}

impl Params {
    pub fn new(args: &[String]) -> Result<Params, &str> {
        if args.len() < 3 {
            return Err("Too few arguments.\nPlease provide a search term and a file name.")
        }
        let pass = args[1].clone();
        let path = args[2].clone();

        Ok(Params { pass, path })
    }
}

pub fn run(params: Params) -> Result<(), Box<dyn Error>> {
    let file = File::open(params.path.clone())?;
    let reader = BufReader::new(file);

    for line in lin_search(&params.pass, reader) {
        println!("Linear search found: {}", line);
    }

    let mut file = File::open(params.path.clone())?;
    let len = i64::try_from(file.seek(SeekFrom::End(0)).unwrap()).unwrap();
    println!("The file is {} bytes long", len);
    let reader = BufReader::new(file);
    println!("Binary search found: {}", bin_search(&params.pass, reader, len));

    Ok(())
}

fn lin_search(pass: &str, reader: BufReader<File>) -> Vec<String> {
    let mut results = Vec::new();

    for line in reader.lines() {
        let s = line.unwrap();
        if s.contains(pass) {
            results.push(s);
            println!("Found string");
        }
    }

    results
}

fn bin_search(pass: &str, mut reader: BufReader<File>, step: i64) -> String {
    reader.seek(SeekFrom::Current(step));

    let mut hash = pass.to_string();

    reader.read_line(&mut hash);
    if hash.cmp(&pass.to_string()) == Ordering::Less {
        hash = bin_search(pass, reader, -step / 2);
    } else if hash.cmp(&pass.to_string()) == Ordering::Greater {
        hash = bin_search(pass, reader, step / 2);
    } 
    hash
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let pass = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(pass, contents));
    }
}
