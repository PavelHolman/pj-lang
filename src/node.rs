use crate::token::Token;
#[derive(Debug)]
pub enum Node {
    Value(Token),
    Unary(UnaryNode),
    Binary(BinaryNode),
}

#[derive(Debug)]
pub struct UnaryNode {
    op: UnaryOperator,
    node: Box<Node>,
}

#[derive(Debug)]
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


#[derive(Debug, PartialEq)]
pub enum Operator {
    Unary(UnaryOperator),
    Binary(BinaryOperator),
    Sentinel,
}

#[derive(Debug)]
pub enum UnaryOperator {
    Not,
    Plus,
    Minus,
}

#[derive(Debug)]
pub enum BinaryOperator {
    Or,
    And,

    Equal,
    Plus,
    Minus,
    Star,
    Slash,
    SlashSlash,
    Percent,

    NotEqual,
    EqualEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}
