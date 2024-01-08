use crate::lexer::tokens::{Token, TokenKind};

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
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
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
        match self.kind {
            TokenKind::Not => Some(UnaryOperator::Not),
            TokenKind::Plus => Some(UnaryOperator::Plus),
            TokenKind::Minus => Some(UnaryOperator::Minus),
            _ => None,
        }
    }
}

impl ToBinary for Token {
    fn to_binary(&self) -> Option<BinaryOperator> {
        match self.kind {
            TokenKind::Or => Some(BinaryOperator::Or),
            TokenKind::And => Some(BinaryOperator::And),
            TokenKind::Equal => Some(BinaryOperator::Equal),
            TokenKind::Plus => Some(BinaryOperator::Add),
            TokenKind::Minus => Some(BinaryOperator::Sub),
            TokenKind::Star => Some(BinaryOperator::Mul),
            TokenKind::Slash => Some(BinaryOperator::Div),
            TokenKind::SlashSlash => Some(BinaryOperator::DivDiv),
            TokenKind::Percent => Some(BinaryOperator::Mod),
            TokenKind::NotEqual => Some(BinaryOperator::NotEqual),
            TokenKind::EqualEqual => Some(BinaryOperator::EqualEqual),
            TokenKind::Less => Some(BinaryOperator::Less),
            TokenKind::LessEqual => Some(BinaryOperator::LessEqual),
            TokenKind::Greater => Some(BinaryOperator::Greater),
            TokenKind::GreaterEqual => Some(BinaryOperator::GreaterEqual),
            _ => None
        }
    }
}
