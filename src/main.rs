mod codegen;
mod lexer;
mod parser;

use codegen::generate;
use lexer::tokenize;
use parser::parse;
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

enum Stage {
    Lex,
    Parse,
    Codegen,
    Compile,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args();
    let program_name = args.next().unwrap_or_else(|| "crust".to_string());

    let mut stage = Stage::Compile;
    let mut file_path = None;
    for arg in args {
        match arg.as_str() {
            "--lex" => stage = Stage::Lex,
            "--parse" => stage = Stage::Parse,
            "--codegen" => stage = Stage::Codegen,
            _ if arg.starts_with("--") => {
                return Err(format!("unsupported option: {arg}"));
            }
            _ => file_path = Some(arg),
        }
    }

    let Some(file_path) = file_path else {
        return Err(format!(
            "usage: {program_name} [--lex|--parse|--codegen] <file>"
        ));
    };

    let input_path = Path::new(&file_path);
    let contents = fs::read_to_string(input_path)
        .map_err(|err| format!("failed to read input file: {err}"))?;
    let tokens = tokenize(&contents).map_err(|err| format!("failed to tokenize: {err}"))?;

    if matches!(stage, Stage::Lex) {
        return Ok(());
    }

    let program = parse(&tokens).map_err(|err| format!("failed to parse: {err}"))?;

    if matches!(stage, Stage::Parse) {
        return Ok(());
    }

    let assembly =
        generate(&program).map_err(|err| format!("failed to generate assembly: {err:?}"))?;

    if matches!(stage, Stage::Codegen) {
        print!("{assembly}");
        return Ok(());
    }

    let assembly_path = assembly_path(input_path);
    fs::write(&assembly_path, assembly)
        .map_err(|err| format!("failed to write assembly file: {err}"))?;

    let output_path = output_path(input_path);
    link_assembly(&assembly_path, &output_path)?;

    return Ok(());
}

fn link_assembly(assembly_path: &Path, output_path: &Path) -> Result<(), String> {
    let output = Command::new("gcc")
        .arg(assembly_path)
        .arg("-o")
        .arg(output_path)
        .output()
        .map_err(|err| format!("failed to invoke gcc: {err}"))?;

    if !output.status.success() {
        return Err(gcc_error(&output));
    }

    return Ok(());
}

fn gcc_error(output: &std::process::Output) -> String {
    let stderr = String::from_utf8_lossy(&output.stderr);
    if stderr.is_empty() {
        return format!("gcc failed with status: {}", output.status);
    }

    return format!(
        "gcc failed with status: {}\n{}",
        output.status,
        stderr.trim_end()
    );
}

fn assembly_path(file_path: &Path) -> PathBuf {
    let mut path = file_path.to_path_buf();
    path.set_extension("s");
    return path;
}

fn output_path(file_path: &Path) -> PathBuf {
    return match file_path.file_stem() {
        Some(file_stem) => file_path.with_file_name(file_stem),
        None => file_path.to_path_buf(),
    };
}
