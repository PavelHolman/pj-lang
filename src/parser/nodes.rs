use crate::parser::operators::{BinaryOperator, UnaryOperator};
use crate::lexer::tokens::Token;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Node {
    Value(Token),
    Unary(UnaryNode),
    Binary(BinaryNode),
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct UnaryNode {
    op: UnaryOperator,
    node: Box<Node>,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct BinaryNode {
    lhs: Box<Node>,
    op: BinaryOperator,
    rhs: Box<Node>,
}

impl UnaryNode {
    pub(crate) fn new(op: UnaryOperator, node: Node) -> Self {
        Self { op, node: node.into() }
    }
}

impl BinaryNode {
    pub(crate) fn new(op: BinaryOperator, lhs: Node, rhs: Node) -> Self {
        Self { op, lhs: lhs.into(), rhs: rhs.into() }
    }
}
