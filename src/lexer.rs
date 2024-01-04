use std::error::Error;
use std::iter::{Fuse, Peekable};
use std::str::Chars;
use crate::token::Token;

pub(crate) struct LexIter<'a> {
    char_iter: Peekable<Fuse<Chars<'a>>>,
}

impl Iterator for LexIter<'_> {
    type Item = Result<Token, Box<dyn Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lex_token()
    }
}


impl<'a> LexIter<'a> {
    pub fn new(source_code: &'a str) -> Self {
        Self {
            char_iter: source_code.chars().fuse().peekable(),
        }
    }

    fn peek_char(&mut self) -> Option<char> {
        self.char_iter.peek().copied()
    }

    fn consume_char(&mut self) -> Option<char> {
        self.char_iter.next()
    }

    fn consume_white_spaces(&mut self) -> () {
        while let Some(ch) = self.char_iter.next_if(|ch| ch.is_whitespace()) {}
    }

    fn consume_if_eq(&mut self, ch: char) -> Option<char> {
        self.char_iter.next_if_eq(&ch)
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
                if self.consume_if_eq('=').is_some() { Token::EqualEqual } else { Token::Equal }
            }
            '+' => Token::Plus,
            '-' => {
                self.consume_white_spaces();
                if self.consume_if_eq('>').is_some() { Token::RArrow } else { Token::Minus }
            }
            '*' => Token::Star,
            '/' => {
                self.consume_white_spaces();
                if self.consume_if_eq('/').is_some() { Token::SlashSlash } else { Token::Slash }
            }
            '%' => Token::Percent,
            '!' => {
                self.consume_white_spaces();
                if self.consume_if_eq('=').is_some() { Token::NotEqual } else { Token::Not }
            }
            '<' => {
                self.consume_white_spaces();
                if self.consume_if_eq('=').is_some() { return Token::LessEqual; }
                if self.consume_if_eq('-').is_none() { return Token::Less; }
                self.consume_white_spaces();
                if self.consume_if_eq('>').is_some() { Token::LRArrow } else { Token::LArrow }
            }
            '>' => {
                self.consume_white_spaces();
                if self.consume_if_eq('=').is_some() { Token::GreaterEqual } else { Token::Greater }
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
            _ => todo!()
        }
    }
}

#[test]
fn test_simple_lexing() {
    let source_code =
        r#"def foo(x <- Int, y <-> String) -> Int | String:
    return if x > 5 then x else y;

let x = 2 + 2 - 1;
let y = "str";
let z = foo(x, y);

print(x, y, z);"#;

    let tokens = LexIter::new(source_code)
        .map(Result::unwrap)
        .collect::<Vec<Token>>();

    use Token::*;

    assert_eq!(tokens, vec![
        Def, Identifier("foo".to_string()), LParen,
        Identifier("x".to_string()), LArrow, Identifier("Int".to_string()), Comma,
        Identifier("y".to_string()), LRArrow, Identifier("String".to_string()), RParen,
        RArrow, Identifier("Int".to_string()), Pipe, Identifier("String".to_string()), Colon,
        Return, If, Identifier("x".to_string()), Greater, Int(5), Then, Identifier("x".to_string()), Else, Identifier("y".to_string()), Semicolon,
        Let, Identifier("x".to_string()), Equal, Int(2), Plus, Int(2), Minus, Int(1), Semicolon,
        Let, Identifier("y".to_string()), Equal, String("str".to_string()), Semicolon,
        Let, Identifier("z".to_string()), Equal, Identifier("foo".to_string()), LParen,
        Identifier("x".to_string()), Comma, Identifier("y".to_string()), RParen, Semicolon,
        Identifier("print".to_string()), LParen,
        Identifier("x".to_string()), Comma,
        Identifier("y".to_string()), Comma,
        Identifier("z".to_string()), RParen, Semicolon,
    ])
}
