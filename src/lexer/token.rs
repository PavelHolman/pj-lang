#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum TokenKind {
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
    StarStar,
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
    Amp,
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

    // Other
    NewLine,
    Whitespace,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Token {
    kind: TokenKind,
    // TODO: Add Span
}

impl Token {
    pub fn new(kind: TokenKind) -> Self {
        Self { kind }
    }
}
