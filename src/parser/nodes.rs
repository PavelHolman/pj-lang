use crate::parser::operators::{BinaryOperator, UnaryOperator};

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Node {
    Value(ValueNode),
    Unary(UnaryNode),
    Binary(BinaryNode),
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum ValueNode {
    Identifier(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
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
