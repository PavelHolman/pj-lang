#[derive(Debug)]
pub enum Node {
    /* TODO */
}

#[derive(Debug)]
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
