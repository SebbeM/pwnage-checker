use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// The file to search
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
    /// Option to perform binary search (requires sorted file)
    binary: String,
}

fn main() {
    let args = Cli::from_args();
    let f = File::open(&args.path).expect("failed to read file");
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    let content = reader.read_line(&mut line);
    println!("First line: {}", line);
}
