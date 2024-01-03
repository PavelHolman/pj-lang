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
