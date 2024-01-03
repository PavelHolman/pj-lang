use std::error::Error;
use crate::token::Token;

#[derive(Debug)]
pub struct AST {}

pub fn parse(tokens: &[Token]) -> Result<AST, Box<dyn Error>> {
    todo!()
}
