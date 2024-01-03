use std::collections::VecDeque;
use std::error::Error;
use std::iter::Fuse;
use std::str::Chars;
use crate::token::Token;

struct Lexer<'a> {
    chars: Fuse<Chars<'a>>,
    buffer: VecDeque<char>,
}

impl<'a> Lexer<'a> {
    fn new(source_code: &'a str) -> Self {
        Self { chars: source_code.chars().fuse(), buffer: VecDeque::new() }
    }

    fn consume(&mut self) -> Option<char> {
        if !self.buffer.is_empty() {
            self.buffer.pop_front()
        } else {
            self.chars.next()
        }
    }

    fn consume_white_spaces(&mut self) -> () {
        while self.peek().map_or(false, |ch| ch.is_whitespace()) { self.consume(); }
    }

    fn consume_if_next(&mut self, ch: char) -> bool {
        if self.peek().expect("TODO") == ch {
            self.consume().unwrap();
            true
        } else { false }
    }

    fn peek(&mut self) -> Option<char> {
        self.peek_nth(0)
    }

    fn peek_nth(&mut self, n: usize) -> Option<char> {
        while !(n < self.buffer.len()) {
            self.buffer.push_back(self.chars.next()?);
        }

        Some(self.buffer[n])
    }

    fn lex_token(&mut self) -> Result<Option<Token>, Box<dyn Error>> {
        self.consume_white_spaces();
        Ok(self.peek().map(|ch| match ch {
            '_' | 'a'..='z' | 'A'..='Z' => self.lex_ident(),
            '0'..='9' => self.lex_number(),
            '\'' | '\"' => self.lex_string(),
            _ => self.lex_special(),
        }))
    }

    fn lex_ident(&mut self) -> Token {
        let mut ident = String::new();

        while let ch = self.peek().expect("TODO") {
            if !ch.is_alphanumeric() && ch != '_' { break; }
            ident.push(self.consume().unwrap())
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

        while let ch = self.peek().expect("TODO") {
            if !found_dot && ch == '.' {
                found_dot = true;
            } else if !ch.is_numeric() && ch != '_' { break; }
            number.push(self.consume().unwrap());
        }

        if found_dot {
            Token::Float(number.parse::<f64>().expect("TODO"))
        } else {
            Token::Int(number.parse::<i64>().expect("TODO"))
        }
    }

    fn lex_string(&mut self) -> Token {
        let mut str = String::new();

        let quote = self.consume().unwrap();

        while let ch = self.consume().expect("TODO") {
            if ch == '\\' {
                str.push(ch);
                str.push(self.consume().expect("TODO"));
                continue;
            }

            if ch == quote { break; }

            str.push(ch)
        }

        Token::String(str)
    }

    fn lex_special(&mut self) -> Token {
        match self.consume().unwrap() {
            '=' => {
                self.consume_white_spaces();
                if self.consume_if_next('=') { Token::EqualEqual } else { Token::Equal }
            }
            '+' => Token::Plus,
            '-' => {
                self.consume_white_spaces();
                if self.consume_if_next('>') { Token::RArrow } else { Token::Minus }
            }
            '*' => Token::Star,
            '/' => {
                self.consume_white_spaces();
                if self.consume_if_next('/') { Token::SlashSlash } else { Token::Slash }
            }
            '%' => Token::Percent,
            '!' => {
                self.consume_white_spaces();
                if self.consume_if_next('=') { Token::NotEqual } else { Token::Not }
            }
            '<' => {
                self.consume_white_spaces();
                if self.consume_if_next('=') { return Token::LessEqual; }
                if !self.consume_if_next('-') { return Token::Less; }
                self.consume_white_spaces();
                if self.consume_if_next('>') { Token::LRArrow } else { Token::LArrow }
            }
            '>' => {
                self.consume_white_spaces();
                if self.consume_if_next('=') { Token::GreaterEqual } else { Token::Greater }
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
            _ => todo!(),
        }
    }
}

pub fn lex(source_code: &str) -> Result<Vec<Token>, Box<dyn Error>> {
    let mut lexer = Lexer::new(source_code);
    let mut tokens = Vec::<Token>::new();

    while let Some(token) = lexer.lex_token()? { tokens.push(token); }

    Ok(tokens)
}
