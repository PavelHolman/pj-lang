use std::error::Error;
use std::iter::{FusedIterator, Peekable};
use crate::lexer::tokens::{Token, TokenKind};
use crate::parser::expression_parser;
use crate::parser::nodes::Node;

pub struct Parser<I: FusedIterator<Item=Token>> {
    token_iterator: Peekable<I>,
}

impl<I: FusedIterator<Item=Token>> Iterator for Parser<I> {
    type Item = Result<Node, Box<dyn Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.parse_node()
    }
}


impl<I: FusedIterator<Item=Token>> Parser<I> {
    pub fn new(token_iterator: Peekable<I>) -> Self {
        Self { token_iterator }
    }

    fn parse_node(&mut self) -> Option<Result<Node, Box<dyn Error>>> {
        let token = self.token_iterator.peek();

        Some(match token?.kind {
            TokenKind::Def => self.parse_definition(),
            // TokenKind::Let => self.parse_let_statement(),
            _ => self.parse_expression(),
        })
    }

    fn parse_definition(&mut self) -> Result<Node, Box<dyn Error>> {
        todo!("Functions, aliases, structs, enums")
    }

    fn parse_statement(&mut self) -> Result<Node, Box<dyn Error>> {
        // todo!("Let, Return, ...")
    }

    fn parse_expression(&mut self) -> Result<Node, Box<dyn Error>> {
        let parser = expression_parser::ExpressionParser::new(&mut self.token_iterator);
        parser.parse()
    }
}
