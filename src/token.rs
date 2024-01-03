#[derive(Clone, Debug)]
pub enum Token {
    // Names
    Identifier(String),

    // Literals
    Int(i64),
    Bool(bool),
    Float(f64),
    String(String),

    // Keywords
    Let,
    Def,
    Return,
    If,
    Then,
    Else,

    Not,
    Or,
    And,

    // Operators
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

    // Punctuation
    Dot,
    Pipe,
    Comma,
    Colon,
    Semicolon,

    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,

    LArrow,
    LRArrow,
    RArrow,

    // Special characters
    EndOfFile,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Identifier(ident) => write!(f, "{ident}"),
            Token::Int(int) => write!(f, "{int}"),
            Token::Bool(bool) => write!(f, "{bool}"),
            Token::Float(float) => write!(f, "{float}"),
            Token::String(string) => write!(f, "\"{string}\""),

            Token::Let => f.write_str("let"),
            Token::Def => f.write_str("def"),
            Token::Return => f.write_str("return"),
            Token::If => f.write_str("if"),
            Token::Then => f.write_str("then"),
            Token::Else => f.write_str("else"),

            Token::Not => f.write_str("not"),
            Token::Or => f.write_str("or"),
            Token::And => f.write_str("and"),

            Token::Equal => f.write_str("="),
            Token::Plus => f.write_str("+"),
            Token::Minus => f.write_str("-"),
            Token::Star => f.write_str("*"),
            Token::Slash => f.write_str("/"),
            Token::SlashSlash => f.write_str("//"),
            Token::Percent => f.write_str("%"),

            Token::NotEqual => f.write_str("!="),
            Token::EqualEqual => f.write_str("=="),
            Token::Less => f.write_str("<"),
            Token::LessEqual => f.write_str("<="),
            Token::Greater => f.write_str(">"),
            Token::GreaterEqual => f.write_str(">="),

            Token::Dot => f.write_str("."),
            Token::Pipe => f.write_str("|"),
            Token::Comma => f.write_str(","),
            Token::Colon => f.write_str(":\n"),
            Token::Semicolon => f.write_str(";\n"),

            Token::LParen => f.write_str("("),
            Token::RParen => f.write_str(")"),
            Token::LBracket => f.write_str("["),
            Token::RBracket => f.write_str("]"),
            Token::LBrace => f.write_str("{"),
            Token::RBrace => f.write_str("}"),

            Token::LArrow => f.write_str("<-"),
            Token::LRArrow => f.write_str("<->"),
            Token::RArrow => f.write_str("->"),

            Token::EndOfFile => f.write_str("EOF"),
        }
    }
}
