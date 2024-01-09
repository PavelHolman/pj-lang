use std::error::Error;
use std::iter::{FusedIterator, Peekable};
use crate::lexer::tokens::{Token, TokenKind};
use crate::parser::{AST, expression_parser};
use crate::parser::nodes::Node;

pub(crate) struct Parser<I: FusedIterator<Item=Token>> {
    token_iterator: Peekable<I>,
}


impl<I: FusedIterator<Item=Token>> Parser<I> {
    pub(crate) fn new(token_iterator: Peekable<I>) -> Self {
        Self { token_iterator }
    }

    pub(crate) fn parse(mut self) -> Result<AST, Box<dyn Error>> {
        let mut nodes = Vec::<Node>::new();

        while let Some(_) = self.token_iterator.peek() {
            nodes.push(self.parse_node()?);
        }

        Ok(AST { nodes })
    }

    fn parse_node(&mut self) -> Result<Node, Box<dyn Error>> {
        match self.token_iterator.peek().unwrap().kind {
            TokenKind::Def => self.parse_definition(),
            _ => self.parse_statement(),
        }
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
