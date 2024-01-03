use std::collections::VecDeque;
use std::error::Error;
use std::iter::Fuse;
use std::str::Chars;
use crate::token::Token;

pub fn tokenize(contents: &str) -> Vec<Token> {
    todo!()
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
    }
}

pub fn lex(source_code: &str) -> Result<Vec<Token>, Box<dyn Error>> {
    let mut lexer = Lexer::new(source_code);
    let mut tokens = Vec::<Token>::new();

    while let Some(token) = lexer.lex_token()? { tokens.push(token); }

    Ok(tokens)
}
