mod lexer;
mod parser;
mod compiler;

use std::error::Error;
use std::{env, fs};
use log::debug;

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<String>>();

    let in_file_path = args.get(1).ok_or("First argument should be input file path.")?;
    let out_file_path = args.get(2).ok_or("Second argument should be output file path.")?;

    let source_code = fs::read_to_string(in_file_path)?;

    let tokens = lexer::tokenize(&source_code);
    debug!("{tokens:?}");

    let ast = parser::parse(&tokens);
    debug!("{ast:?}");

    let compiled = compiler::compile(&ast);
    fs::write(out_file_path, compiled)?;

    Ok(())
}
