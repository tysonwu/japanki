use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn read_file(fname: std::path::PathBuf) -> std::io::Result<()> {
    let file = File::open(fname)?;
    let reader = BufReader::new(file);
    let pattern = String::from("ash");

    for line in reader.lines() {
        check_line(&pattern, line?);
    }
    Ok(())
}

fn check_line(pattern: &String, line: String) {
    match &line {
        s if s.contains(pattern) => println!("contains {}", pattern),
        _ => (),
    }
}

fn main() {
    let args = Cli::parse();
    let _ = read_file(args.path);
}
