use std::fs::File;
use std::error::Error;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Seek;
use std::io::SeekFrom;

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
    let mut file = File::open(params.path.clone())?;
    let len = file.seek(SeekFrom::End(0)).unwrap();
    let reader = BufReader::new(file);

    println!("The file is {} bytes long", len);

    for line in search(&params.pass, reader) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search(pass: &str, reader: BufReader<File>) -> Vec<String> {
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
