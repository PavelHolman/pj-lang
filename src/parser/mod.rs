use std::error::Error;
use std::iter::{FusedIterator, Peekable};
use node::Node;
use crate::lexer::TokenIterator;
use crate::lexer::token::Token;

pub mod node;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct AST {
    nodes: Vec<Node>,
}

struct Parser<I: FusedIterator<Item=Token>> {
    token_iterator: Peekable<I>,
}


impl<I: FusedIterator<Item=Token>> Parser<I> {
    fn new(token_iterator: Peekable<I>) -> Self
    {
        Self { token_iterator }
    }

    fn parse(mut self) -> Result<AST, Box<dyn Error>> {
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

pub fn parse<I>(token_iterator: I) -> Result<AST, Box<dyn Error>>
    where I: Iterator<Item=Token>
{
    let token_iterator = token_iterator.fuse().peekable();
    let parser = Parser::new(token_iterator);
    let ast = parser.parse()?;

    Ok(ast)
}

    let nodes = parser.parse()?;

    Ok(AST { nodes })
}
