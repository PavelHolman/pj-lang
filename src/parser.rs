use std::error::Error;
use std::iter::Peekable;
use crate::lexer::LexIter;
use crate::node::{Node, Operator};
use crate::token::Token;

#[derive(Debug)]
pub struct AST {
    nodes: Vec<Node>,
}

struct Parser<'a> {
    lexer: Peekable<LexIter<'a>>,
    operator_stack: Vec<Operator>,
    operand_stack: Vec<Node>,
}

impl<'a> Parser<'a> {
    fn new(lexer: LexIter<'a>) -> Self {
        Self {
            lexer: lexer.peekable(),
            operator_stack: vec![Operator::Sentinel],
            operand_stack: vec![],
        }
    }

    fn parse(mut self) -> Result<Vec<Node>, Box<dyn Error>> {
        let mut nodes = Vec::<Node>::new();

        while self.lexer.peek().is_some() {
            let node = self.parse_primary_node();
            nodes.push(node?);
        }

        Ok(nodes)
    }

    fn parse_primary_node(&mut self) -> Result<Node, Box<dyn Error>> {
        let tok = self.lexer.next().expect("Should only be called if peeked Some(..).")?;
        match tok {
            Token::Let => self.parse_variable_decl(),
            Token::Def => self.parse_function_decl(),
            Token::If => self.parse_if_statement(),
            _ => self.parse_expression(),
        }

        Ok(self.operand_stack.pop().expect("Proper parsing should always keep resulting operand on the stack."))
    }

    fn parse_variable_decl(&mut self) -> () { todo!() }

    fn parse_function_decl(&mut self) -> () { todo!() }

    fn parse_if_statement(&mut self) -> () { todo!() }

    fn parse_expression(&mut self) -> () { todo!() }
}

pub fn parse(source_code: &str) -> Result<AST, Box<dyn Error>> {
    let mut nodes = Vec::<Node>::new();

    let lexer = LexIter::new(&source_code);
    let mut parser = Parser::new(lexer);

    let nodes = parser.parse()?;

    Ok(AST { nodes })
}
