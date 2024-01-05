use std::cmp::Ordering;
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

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Not,
    Plus,
    Minus,
}

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    Or,
    And,

    Equal,
    Add,
    Sub,
    Mul,
    Div,
    DivDiv,
    Mod,

    NotEqual,
    EqualEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

#[derive(PartialOrd, PartialEq)]
pub enum Precedence {
    Sentinel,
    Assign,
    Or,
    And,
    Eq,
    Ord,
    // Low: Add, Sub
    ArithLow,
    // Mid: Mul, Div, Mod
    ArithHigh,
    Unary,
}

impl Operator {
    fn precedence(&self) -> Precedence {
        match self {
            Operator::Unary(_) => Precedence::Unary,
            Operator::Binary(bin) => match bin {
                BinaryOperator::Or => Precedence::Or,
                BinaryOperator::And => Precedence::And,
                BinaryOperator::Equal => Precedence::Assign,
                BinaryOperator::Add | BinaryOperator::Sub => Precedence::ArithLow,
                BinaryOperator::Mul | BinaryOperator::Div |
                BinaryOperator::DivDiv | BinaryOperator::Mod => Precedence::ArithHigh,
                BinaryOperator::NotEqual | BinaryOperator::EqualEqual => Precedence::Eq,
                BinaryOperator::Less | BinaryOperator::LessEqual |
                BinaryOperator::Greater | BinaryOperator::GreaterEqual => Precedence::Ord
            }
            Operator::Sentinel => Precedence::Sentinel,
        }
    }
}

impl PartialOrd for Operator {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.precedence().partial_cmp(&other.precedence())
    }
}


pub trait ToUnary {
    fn to_unary(&self) -> Option<UnaryOperator>;
}

pub trait ToBinary {
    fn to_binary(&self) -> Option<BinaryOperator>;
}

impl ToUnary for Token {
    fn to_unary(&self) -> Option<UnaryOperator> {
        match self {
            Token::Not => Some(UnaryOperator::Not),
            Token::Plus => Some(UnaryOperator::Plus),
            Token::Minus => Some(UnaryOperator::Minus),
            _ => None,
        }
    }
}

impl ToBinary for Token {
    fn to_binary(&self) -> Option<BinaryOperator> {
        match self {
            Token::Or => Some(BinaryOperator::Or),
            Token::And => Some(BinaryOperator::And),
            Token::Equal => Some(BinaryOperator::Equal),
            Token::Plus => Some(BinaryOperator::Add),
            Token::Minus => Some(BinaryOperator::Sub),
            Token::Star => Some(BinaryOperator::Mul),
            Token::Slash => Some(BinaryOperator::Div),
            Token::SlashSlash => Some(BinaryOperator::DivDiv),
            Token::Percent => Some(BinaryOperator::Mod),
            Token::NotEqual => Some(BinaryOperator::NotEqual),
            Token::EqualEqual => Some(BinaryOperator::EqualEqual),
            Token::Less => Some(BinaryOperator::Less),
            Token::LessEqual => Some(BinaryOperator::LessEqual),
            Token::Greater => Some(BinaryOperator::Greater),
            Token::GreaterEqual => Some(BinaryOperator::GreaterEqual),
            _ => None
        }
    }
}
