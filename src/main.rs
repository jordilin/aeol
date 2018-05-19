use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

// Add end of line if missing. This will execute under acmego
// for any file not identified with a formatter.
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected source file");
    }
    let f = File::open(&args[1]).unwrap();
    let reader = BufReader::new(f);
    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}
