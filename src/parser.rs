use std::error::Error;
use std::iter::Peekable;
use crate::lexer::LexIter;
use crate::node::{BinaryNode, Node, Operator, ToBinary, ToUnary, UnaryNode};
use crate::token::Token;

#[derive(Debug)]
pub struct AST {
    nodes: Vec<Node>,
}

struct Parser<'a> {
    lexer: Peekable<LexIter<'a>>,
}

// TODO: ExpressionParser as well as many other classes across the project
// TODO: would greatly benefit from some modularization and refactoring ;)
struct ExpressionParser<'a, 'b> {
    lexer: &'b mut Peekable<LexIter<'a>>,
    operator_stack: Vec<Operator>,
    operand_stack: Vec<Node>,
}

impl<'a, 'b> ExpressionParser<'a, 'b>
    where 'a: 'b
{
    fn new(lexer: &'b mut Peekable<LexIter<'a>>) -> Self {
        Self {
            lexer,
            operator_stack: vec![Operator::Sentinel],
            operand_stack: vec![],
        }
    }

    fn parse(mut self) -> Result<Node, Box<dyn Error>> {
        let node = self.parse_expr()?;

        if !matches!(self.lexer.peek(), Some(&Ok(Token::Semicolon))) { todo!("Error") }

        Ok(node)
    }

    fn parse_expr(&mut self) -> Result<Node, Box<dyn Error>> {
        self.parse_primary()?;

        while let Some(tok) = self.lexer.next() {
            let tok = tok?;
            let binary_op = tok.to_binary().unwrap();
            self.push_operator(Operator::Binary(binary_op));
            self.parse_primary()?;
        }

        while !matches!(self.operator_stack.last().unwrap(), Operator::Sentinel) {
            self.pop_operator();
        }

        Ok(self.operand_stack.pop().unwrap())
    }

    fn parse_primary(&mut self) -> Result<(), Box<dyn Error>> {
        let tok = self.lexer.next().unwrap()?;

        match tok {
            Token::Identifier(_) | Token::Int(_) | Token::Bool(_) | Token::Float(_) | Token::String(_) => {
                self.operand_stack.push(Node::Value(tok.clone())); // TODO: Don't clone
            }

            Token::LParen => {
                self.operator_stack.push(Operator::Sentinel);
                self.parse_expr()?;
                if !matches!(self.lexer.peek(), Some(&Ok(Token::RParen))) { todo!("Error") }
            }

            _ if tok.to_unary().is_some() => {
                let unary_op = tok.to_unary().unwrap();
                self.push_operator(Operator::Unary(unary_op));
                self.parse_primary()?;
            }

            _ => { todo!("Probably error for others, look into it") }
        }

        Ok(())
    }

    fn push_operator(&mut self, operator: Operator) {
        while *self.operator_stack.last().unwrap() > operator {
            self.pop_operator();
        }
        self.operator_stack.push(operator);
    }

    fn pop_operator(&mut self) {
        match self.operator_stack.pop().unwrap() {
            Operator::Binary(binary_operator) => {
                let t1 = self.operand_stack.pop().unwrap();
                let t0 = self.operand_stack.pop().unwrap();
                self.operand_stack.push(Node::Binary(BinaryNode::new(binary_operator, t0, t1)));
            }
            Operator::Unary(unary_node) => {
                let t = self.operand_stack.pop().unwrap();
                self.operand_stack.push(Node::Unary(UnaryNode::new(unary_node, t)));
            }
            _ => { /* Maybe unreachable? */ }
        }
    }
}

impl<'a> Parser<'a> {
    fn new(lexer: LexIter<'a>) -> Self {
        Self { lexer: lexer.peekable() }
    }

    fn parse(mut self) -> Result<Vec<Node>, Box<dyn Error>> {
        let mut nodes = Vec::<Node>::new();

        while self.lexer.peek().is_some() {
            let node = self.parse_primary_node()?;
            nodes.push(node);
        }

        Ok(nodes)
    }

    fn parse_primary_node(&mut self) -> Result<Node, Box<dyn Error>> {
        let tok = self.lexer.next().unwrap()?;
        let node = match tok {
            Token::Let => self.parse_variable_decl(),
            Token::Def => self.parse_function_decl(),
            Token::If => self.parse_if_statement(),
            _ => self.parse_expression(),
        }?;

        Ok(node)
    }

    fn parse_variable_decl(&mut self) -> Result<Node, Box<dyn Error>> { todo!() }

    fn parse_function_decl(&mut self) -> Result<Node, Box<dyn Error>> { todo!() }

    fn parse_if_statement(&mut self) -> Result<Node, Box<dyn Error>> { todo!() }

    fn parse_expression(&mut self) -> Result<Node, Box<dyn Error>> {
        let mut expression_parser = ExpressionParser::new(&mut self.lexer);
        expression_parser.parse()
    }
}

pub fn parse(source_code: &str) -> Result<AST, Box<dyn Error>> {
    let lexer = LexIter::new(&source_code);
    let mut parser = Parser::new(lexer);

    let nodes = parser.parse()?;

    Ok(AST { nodes })
}
