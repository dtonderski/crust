mod lexer;
mod parser;

use lexer::tokenize;
use parser::parse;
use std::{env, fs::File, io::Read};

fn main() {
    let mut args = env::args();
    let program_name = args.next().unwrap_or_else(|| "crust".to_string());

    let mut should_parse = true;
    let mut file_path = None;
    for arg in args {
        match arg.as_str() {
            "--lex" => should_parse = false,
            "--parse" => should_parse = true,
            _ if arg.starts_with("--") => {
                eprintln!("unsupported option: {arg}");
                std::process::exit(1);
            }
            _ => file_path = Some(arg),
        }
    }

    let Some(file_path) = file_path else {
        eprintln!("usage: {program_name} [--lex] <file>");
        std::process::exit(1);
    };

    let contents = match read_file(&file_path) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("failed to read input file: {err}");
            std::process::exit(1);
        }
    };

    let tokens = match tokenize(&contents) {
        Ok(tokens) => tokens,
        Err(err) => {
            eprintln!("failed to tokenize: {err}");
            std::process::exit(1);
        }
    };

    if should_parse {
        if let Err(err) = parse(&tokens) {
            eprintln!("failed to parse: {err}");
            std::process::exit(1);
        }
    }
}

fn read_file(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    return Ok(contents);
}
