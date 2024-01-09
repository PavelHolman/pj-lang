use std::error::Error;
use nodes::Node;
use parser::Parser;
use crate::lexer::Lexer;
use crate::lexer::tokens::{Token, TokenKind};

pub mod nodes;
pub mod parser;
mod operators;
mod expression_parser;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct AST {
    nodes: Vec<Node>,
}

pub fn parse(tokens: impl IntoIterator<Item=Token>) -> Result<AST, Box<dyn Error>> {
    let token_iterator = tokens
        .into_iter()
        .filter(|token: &Token| !matches!(token.kind, TokenKind::Whitespace | TokenKind::NewLine))
        .fuse()
        .peekable();

    let parser = Parser::new(token_iterator);
    let ast = parser.parse()?;

    Ok(ast)
}

#[test]
fn test_simple_parsing() {
    let source_code =
        "2 + (1 + 3);\
        x = 2 + 1;\
        x = x + 5;";


    let lexer = Lexer::new(source_code).map(Result::unwrap);
    let ast1 = parse(lexer).unwrap();

    let tokens = Lexer::new(source_code).map(Result::unwrap).collect::<Vec<_>>();
    let ast2 = parse(tokens).unwrap();

    assert_eq!(ast1, AST { nodes: vec![] });
    assert_eq!(ast2, AST { nodes: vec![] });
}
