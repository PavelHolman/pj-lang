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
        // let tok = self.token_iterator.next().unwrap();
        // let node = match tok {
        //     Token::Let => self.parse_variable_decl(),
        //     Token::Def => self.parse_function_decl(),
        //     Token::If => self.parse_if_statement(),
        //     _ => self.parse_expression(),
        // }?;
        //
        // Ok(node)
        panic!()
    }

    fn parse_variable_decl(&mut self) -> Result<Node, Box<dyn Error>> { todo!() }

    fn parse_function_decl(&mut self) -> Result<Node, Box<dyn Error>> { todo!() }

    fn parse_if_statement(&mut self) -> Result<Node, Box<dyn Error>> { todo!() }

    fn parse_expression(&mut self) -> Result<Node, Box<dyn Error>> {
        let mut expression_parser = ExpressionParser::new(&mut self.lexer);
        expression_parser.parse()
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
