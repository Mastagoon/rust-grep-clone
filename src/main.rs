use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    // find file
    let content = fs::read_to_string(args.path).expect("File not found.");
    println!("{}", content)
}
