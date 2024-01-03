#[derive(Debug)]
pub enum Token {}
use crate::token::Token;

pub fn tokenize(contents: &str) -> Vec<Token> {
    todo!()
}

pub fn lex(source_code: &str) -> Result<Vec<Token>, Box<dyn std::error::Error>> {
}