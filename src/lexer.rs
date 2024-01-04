use std::error::Error;
use std::iter::{Fuse, Peekable};
use std::str::Chars;
use crate::token::Token;

trait PeekIterator {
    type Item;

    fn peek(&mut self) -> Option<Self::Item>;
}

struct CharIter<'a> {
    inner: Peekable<Fuse<Chars<'a>>>,
    ch: Option<char>,
    line: usize,
    col: usize,
}

impl Iterator for CharIter<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let ch = self.inner.next();
        if let Some(ch) = ch {
            self.col += 1;
            if ch == '\n' {
                self.col = 1;
                self.line += 1;
            }
        }

        ch
    }
}

impl PeekIterator for CharIter<'_> {
    type Item = char;

    fn peek(&mut self) -> Option<Self::Item> {
        self.inner.peek().copied()
    }
}


pub(crate) struct LexIter<'a> {
    chars: CharIter<'a>,
    next_res: Option<Result<Token, Box<dyn Error>>>,
}

impl Iterator for LexIter<'_> {
    type Item = Result<Token, Box<dyn Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lex_token()
    }
}

impl PeekIterator for LexIter<'_> {
    type Item = Result<Token, Box<dyn Error>>;

    fn peek(&mut self) -> Option<Self::Item> {
        // self.next_res.or(self.next())
        todo!()
    }
}


impl<'a> LexIter<'a> {
    pub fn new(source_code: &'a str) -> Self {
        Self {
            chars: CharIter {
                inner: source_code.chars().fuse().peekable(),
                ch: None,
                line: 1,
                col: 0,
            },
            next_res: None,
        }
    }

    fn peek_char(&mut self) -> Option<char> {
        self.chars.peek()
    }

    fn consume_char(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn consume_white_spaces(&mut self) -> () {
        while self.chars.peek().map_or(false, char::is_whitespace) { self.chars.next(); }
    }

    fn consume_if_eq(&mut self, ch: char) -> bool {
        if self.peek_char().expect(/* TODO */"") == ch {
            self.consume_char().unwrap();
            true
        } else { false }
    }

    fn lex_token(&mut self) -> Option<Result<Token, Box<dyn Error>>> {
        self.consume_white_spaces();
        Some(Ok(match self.peek_char()? {
            '_' | 'a'..='z' | 'A'..='Z' => self.lex_ident(),
            '0'..='9' => self.lex_number(),
            '\'' | '\"' => self.lex_string(),
            _ => self.lex_special(),
        }))
    }

    fn lex_ident(&mut self) -> Token {
        let mut ident = String::new();

        while let ch = self.peek_char().expect(/* TODO */"") {
            if !ch.is_alphanumeric() && ch != '_' { break; }
            ident.push(self.consume_char().unwrap())
        }

        match ident.as_str() {
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),

            "let" => Token::Let,
            "def" => Token::Def,
            "return" => Token::Return,
            "if" => Token::If,
            "then" => Token::Then,
            "else" => Token::Else,

            "not" => Token::Not,
            "or" => Token::Or,
            "and" => Token::And,

            _ => Token::Identifier(ident),
        }
    }

    fn lex_number(&mut self) -> Token {
        let mut number = String::new();
        let mut found_dot = false;

        while let ch = self.peek_char().expect(/* TODO */"") {
            if !found_dot && ch == '.' {
                found_dot = true;
            } else if !ch.is_numeric() && ch != '_' { break; }
            number.push(self.consume_char().unwrap());
        }

        if found_dot {
            Token::Float(number.parse::<f64>().expect(/* TODO */""))
        } else {
            Token::Int(number.parse::<i64>().expect(/* TODO */""))
        }
    }

    fn lex_string(&mut self) -> Token {
        let mut str = String::new();

        let quote = self.consume_char().unwrap();

        while let ch = self.consume_char().expect(/* TODO */"") {
            if ch == '\\' {
                str.push(ch);
                str.push(self.consume_char().expect(/* TODO */""));
                continue;
            }

            if ch == quote { break; }

            str.push(ch)
        }

        Token::String(str)
    }

    fn lex_special(&mut self) -> Token {
        match self.consume_char().unwrap() {
            '=' => {
                self.consume_white_spaces();
                if self.consume_if_eq('=') { Token::EqualEqual } else { Token::Equal }
            }
            '+' => Token::Plus,
            '-' => {
                self.consume_white_spaces();
                if self.consume_if_eq('>') { Token::RArrow } else { Token::Minus }
            }
            '*' => Token::Star,
            '/' => {
                self.consume_white_spaces();
                if self.consume_if_eq('/') { Token::SlashSlash } else { Token::Slash }
            }
            '%' => Token::Percent,
            '!' => {
                self.consume_white_spaces();
                if self.consume_if_eq('=') { Token::NotEqual } else { Token::Not }
            }
            '<' => {
                self.consume_white_spaces();
                if self.consume_if_eq('=') { return Token::LessEqual; }
                if !self.consume_if_eq('-') { return Token::Less; }
                self.consume_white_spaces();
                if self.consume_if_eq('>') { Token::LRArrow } else { Token::LArrow }
            }
            '>' => {
                self.consume_white_spaces();
                if self.consume_if_eq('=') { Token::GreaterEqual } else { Token::Greater }
            }
            '.' => Token::Dot,
            '|' => Token::Pipe,
            ',' => Token::Comma,
            ':' => Token::Colon,
            ';' => Token::Semicolon,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '[' => Token::LBracket,
            ']' => Token::RBracket,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            ch => panic!("Failed to parse character '{}' on line {}:{}", ch, self.chars.line, self.chars.col)
        }
    }
}

pub fn lex(source_code: &str) -> Result<Vec<Token>, Box<dyn Error>> {
    let mut tokens = Vec::<Token>::new();

    let mut lexer = Lexer::new(source_code);
    while let Some(token) = lexer.lex_token()? { tokens.push(token); }

    Ok(tokens)
}
