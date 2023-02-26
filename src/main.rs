// use clap::Parser;
// use std::fs;
// use std::io::prelude::BufRead;

// #[derive(Parser, Debug)]
// struct Cli {
// pattern: String,
// path: std::path::PathBuf,
// }

#[derive(Debug)]
struct Complex {
    real: i32,
    imaginary: i32,
}

impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {}i", self.real, self.imaginary)
    }
}

fn main() {
    let val = Complex {
        imaginary: 15,
        real: 12,
    };
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );
    let ys: [i32; 500] = [0; 500];
    let v = ys.get(5).expect("pffft");
    println!("{:?}", long_tuple);
    println!("{:?}", val);
    println!("{}", val);
    // let args = Cli::parse();
    // find file
    // let f = fs::File::open(&args.path).expect("File not found.");
    // let mut reader = std::io::BufReader::new(f);
    // let mut line = String::new();
    // let len = reader.read_line(&mut line)?;
    //
    // let content = fs::read_to_string(args.path).expect("File not found.");
    // let i = 1;
    // for line in content.lines() {
    // if line.contains(&args.pattern) {
    // println!("{}: {}", i, line);
    // }
    // }
    // Ok(())
}
