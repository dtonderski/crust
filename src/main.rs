mod lexer;

use lexer::tokenize;
use std::{env, fs::File, io::Read};

fn main() {
    let mut args = env::args();
    let program_name = args.next().unwrap_or_else(|| "crust".to_string());

    let mut file_path = None;
    for arg in args {
        match arg.as_str() {
            "--lex" => {}
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

    if let Err(err) = tokenize(&contents) {
        eprintln!("failed to tokenize: {err:?}");
        std::process::exit(1);
    }
}

fn read_file(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    return Ok(contents);
}
