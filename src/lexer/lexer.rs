use std::error::Error;
use std::iter::{Fuse, Peekable};
use std::str::Chars;
use crate::lexer::tokens::{Token, TokenKind};

pub struct Lexer<'a> {
    char_iter: Peekable<Fuse<Chars<'a>>>,
}

impl Iterator for Lexer<'_> {
    type Item = Result<Token, Box<dyn Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}


impl<'a> Lexer<'a> {
    pub fn new(source_code: &'a str) -> Self {
        Self {
            char_iter: source_code.chars().fuse().peekable(),
        }
    }

    fn next_token(&mut self) -> Option<Result<Token, Box<dyn Error>>> {
        let kind = self.peek_char().map(|c| {
            match c {
                _ if Self::is_newline_start(c) => self.consume_newline(),
                _ if Self::is_whitespace_start(c) => self.consume_whitespace(),
                _ if Self::is_number_start(c) => self.consume_number(),
                _ if Self::is_identifier_start(c) => self.consume_identifier(),
                _ if Self::is_string_start(c) => self.consume_string(),
                _ if Self::is_punctuation_start(c) => self.consume_punctuation(),
                _ => todo!("Error: Unknown token"),
            }
        });

        kind.map(|r| r.map(Token::new))
    }


    fn is_newline_start(c: char) -> bool {
        c == '\n'
    }

    fn is_whitespace_start(c: char) -> bool {
        c.is_whitespace()
    }

    fn is_number_start(c: char) -> bool {
        c.is_digit(10)
    }

    fn is_number_cont(c: char) -> bool {
        c.is_digit(10) || c == '.' || c == '_'
    }

    fn is_identifier_start(c: char) -> bool {
        c == '_' || c.is_alphabetic()
    }

    fn is_identifier_cont(c: char) -> bool {
        c == '_' || c.is_alphanumeric()
    }

    fn is_string_start(c: char) -> bool {
        matches!(c, '\'' | '\"')
    }

    fn is_punctuation_start(c: char) -> bool {
        matches!(c, '=' | '+' | '-' | '*' | '/' | '%'
                  | '!' | '<' | '>' | '&' | '|'
                  | '.' | ',' | ':' | ';'
                  | '(' | ')' | '[' | ']' | '{' | '}')
    }

    fn peek_char(&mut self) -> Option<char> {
        self.char_iter.peek().copied()
    }

    fn consume_char(&mut self) -> Option<char> {
        self.char_iter.next()
    }

    fn consumed_next_if_eq(&mut self, c: char) -> bool {
        self.char_iter.next_if_eq(&c).is_some()
    }

    fn consume_newline(&mut self) -> Result<TokenKind, Box<dyn Error>> {
        self.consume_char();
        Ok(TokenKind::NewLine) // TODO: deal with \r\n etc.
    }

    fn consume_whitespace(&mut self) -> Result<TokenKind, Box<dyn Error>> {
        self.consume_char();
        Ok(TokenKind::Whitespace) // TODO: deal with tabs etc.
    }

    fn consume_identifier(&mut self) -> Result<TokenKind, Box<dyn Error>> {
        let mut identifier = String::new();
        while let Some(c) = self.peek_char() {
            if Self::is_identifier_cont(c) {
                self.consume_char();
                identifier.push(c);
            } else {
                break;
            }
        }

        Ok(match identifier.as_str() {
            "true" => TokenKind::Bool(true),
            "false" => TokenKind::Bool(false),

            "let" => TokenKind::Let,
            "def" => TokenKind::Def,
            "if" => TokenKind::If,
            "then" => TokenKind::Then,
            "else" => TokenKind::Else,
            "return" => TokenKind::Return,
            _ => TokenKind::Identifier(identifier)
        })
    }

    fn consume_number(&mut self) -> Result<TokenKind, Box<dyn Error>> {
        let mut number = String::new();
        let mut found_dot = false;
        while let Some(c) = self.peek_char() {
            if Self::is_number_cont(c) {
                self.consume_char();
                number.push(c);
            } else if c == '.' && !found_dot {
                found_dot = true;
                number.push(c);
            } else {
                if c != '_' { break; }
                // TODO: Error for letters or other non allowed chars immediately after number
            }
        }

        if found_dot {
            Ok(TokenKind::Float(number.parse::<f64>()?))
        } else {
            Ok(TokenKind::Int(number.parse::<i64>()?))
        }
    }
    fn consume_string(&mut self) -> Result<TokenKind, Box<dyn Error>> {
        let mut string = String::new();
        let quote = self.consume_char().unwrap();

        while let Some(mut c) = self.consume_char() {
            if c == quote { return Ok(TokenKind::String(string)); }

            if c == '\\' {
                if self.peek_char().is_none() { break; }
                c = self.consume_char().unwrap();
            }

            string.push(c);
        }

        todo!("Error")
    }
    fn consume_punctuation(&mut self) -> Result<TokenKind, Box<dyn Error>> {
        Ok(match self.consume_char().unwrap() {
            '=' => if self.consumed_next_if_eq('=') { TokenKind::EqualEqual } else { TokenKind::Equal },

            '+' => TokenKind::Plus,
            '-' => if self.consumed_next_if_eq('>') { TokenKind::RArrow } else { TokenKind::Minus },
            '*' => if self.consumed_next_if_eq('*') { TokenKind::StarStar } else { TokenKind::Star },
            '/' => if self.consumed_next_if_eq('/') { TokenKind::SlashSlash } else { TokenKind::Slash },
            '%' => TokenKind::Percent,

            '!' => if self.consumed_next_if_eq('=') { TokenKind::NotEqual } else { TokenKind::Not },
            '<' => if self.consumed_next_if_eq('=') {
                TokenKind::LessEqual
            } else if !self.consumed_next_if_eq('-') {
                TokenKind::Less
            } else if self.consumed_next_if_eq('>') {
                TokenKind::LRArrow
            } else {
                TokenKind::LArrow
            }
            '>' => if self.consumed_next_if_eq('=') { TokenKind::GreaterEqual } else { TokenKind::Greater },

            '&' => if self.consumed_next_if_eq('&') { TokenKind::And } else { TokenKind::Amp },
            '|' => if self.consumed_next_if_eq('|') { TokenKind::Or } else { TokenKind::Pipe },

            '.' => TokenKind::Dot,
            ',' => TokenKind::Comma,
            ':' => TokenKind::Colon,
            ';' => TokenKind::Semicolon,

            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '[' => TokenKind::LBracket,
            ']' => TokenKind::RBracket,
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,

            _ => unreachable!("Only punctuator starting chars are matched.")
        })
    }
}
