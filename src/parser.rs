use std::error::Error;
use crate::lexer::Token;

#[derive(Debug)]
pub struct AST {}

pub fn parse(tokens: &[Token]) -> Result<AST, Box<dyn Error>> {
    todo!()
}
