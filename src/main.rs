mod lexer;
mod parser;
mod compiler;

use std::error::Error;
use std::{env, fs};

fn compile(source_code: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let tokens = lexer::lex(&source_code)?;

    let ast = parser::parse(&tokens)?;

    Ok(compiler::compile(&ast)?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<String>>();

    let in_file_path = args.get(1).ok_or("First argument should be input file path.")?;
    let out_file_path = args.get(2).ok_or("Second argument should be output file path.")?;

    let source_code = fs::read_to_string(in_file_path)?;

    let compiled = compile(&source_code)?;
    fs::write(out_file_path, compiled)?;

    Ok(())
}
