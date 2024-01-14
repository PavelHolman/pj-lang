use std::error::Error;
use nodes::Node;
use parser::Parser;
use crate::lexer::lex;
use crate::lexer::tokens::{Token, TokenKind};
use crate::parser::nodes::{BinaryNode, ValueNode};
use crate::parser::operators::BinaryOperator;

pub mod nodes;
pub mod parser;
mod operators;
mod expression_parser;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct AST {
    nodes: Vec<Node>,
}

pub fn parse(tokens: impl IntoIterator<Item=Token>) -> impl Iterator<Item=Result<Node, Box<dyn Error>>> {
    let token_iterator = tokens
        .into_iter()
        .filter(|token: &Token| !matches!(token.kind, TokenKind::Whitespace | TokenKind::NewLine))
        .fuse()
        .peekable();

    let parser = Parser::new(token_iterator);
    parser
}

#[test]
fn test_simple_parsing() {
    let source_code =
        "2 + (1 + 3);\
        (2 + 1) + 3;";

    let nodes = parse(lex(source_code.chars()).map(Result::unwrap)).map(Result::unwrap).collect::<Vec<Node>>();

    assert_eq!(nodes, vec![Node::Binary(BinaryNode::new(BinaryOperator::Add,
                                                       Node::Value(ValueNode::Int(2)),
                                                       Node::Binary(BinaryNode::new(
                                                           BinaryOperator::Add,
                                                           Node::Value(ValueNode::Int(1)),
                                                           Node::Value(ValueNode::Int(3)))))),
                          Node::Binary(BinaryNode::new(BinaryOperator::Add,
                                                       Node::Binary(BinaryNode::new(
                                                           BinaryOperator::Add,
                                                           Node::Value(ValueNode::Int(2)),
                                                           Node::Value(ValueNode::Int(1)))),
                                                       Node::Value(ValueNode::Int(3))))]);
}
