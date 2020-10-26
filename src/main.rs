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
    println!("{}", args.binary);
}
