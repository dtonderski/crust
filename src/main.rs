mod lexer;

use lexer::tokenize;
use std::{env, fs::File, io::Read};

fn main() {
    let mut args = env::args();
    let program_name = args.next().unwrap_or_else(|| "crust".to_string());
    let Some(file_path) = args.next() else {
        eprintln!("usage: {program_name} <file>");
        std::process::exit(1);
    };

    let contents = read_file(&file_path).expect("failed to read input file");

    let e = tokenize(&contents).expect("failed to tokenize");

    dbg!(e);
}

fn read_file(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    return Ok(contents);
}
