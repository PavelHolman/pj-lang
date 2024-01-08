use std::error::Error;
use std::iter::{FusedIterator, Peekable};
use crate::lexer::tokens::Token;
use crate::parser::AST;
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
        todo!("Parse node - statement/expression")
    }

    fn parse_expression(&mut self) -> Result<Node, Box<dyn Error>> {
        todo!("Call expression parser")
    }
}
