use clap::Parser;
use std::fs;
use std::io::prelude::BufRead;

#[derive(Parser, Debug)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    // find file
    let f = fs::File::open(&args.path).expect("File not found.");
    let reader = std::io::BufReader::new(f);
    let mut i = 1;
    for line in reader.lines() {
        let line = line.expect("Error reading line");
        if line.contains(&args.pattern) {
            println!("line: {}: {}", i, line);
        }
        i += 1;
    }
}
