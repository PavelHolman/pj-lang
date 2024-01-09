use std::error::Error;
use std::iter::Peekable;
use crate::lexer::tokens::{Token, TokenKind};
use crate::parser::nodes::{BinaryNode, Node, UnaryNode, ValueNode};
use crate::parser::operators::{Operator, ToBinary, ToUnary};

pub(crate) struct ExpressionParser<'a, I: Iterator<Item=Token>> {
    token_iterator: &'a mut Peekable<I>,
    operator_stack: Vec<Operator>,
    operand_stack: Vec<Node>,
}

impl<'a, I: Iterator<Item=Token>> ExpressionParser<'a, I>
{
    pub(crate) fn new(token_iterator: &'a mut Peekable<I>) -> Self {
        Self {
            token_iterator,
            operator_stack: vec![Operator::Sentinel],
            operand_stack: vec![],
        }
    }

    pub(crate) fn parse(mut self) -> Result<Node, Box<dyn Error>> {
        self.parse_expr()?;
        if !matches!(self.token_iterator.peek().ok_or("Error")?.kind, TokenKind::Semicolon) { todo!("Error") }

        Ok(self.operand_stack.pop().unwrap())
    }

    fn parse_expr(&mut self) -> Result<(), Box<dyn Error>> {
        self.parse_primary()?;

        while let Some(binary_op) = self.token_iterator.peek().ok_or("Error")?.to_binary() {
            self.push_operator(Operator::Binary(binary_op));
            self.parse_primary()?;
            self.token_iterator.next();
        }

        while !matches!(self.operator_stack.last().unwrap(), Operator::Sentinel) {
            self.pop_operator();
        }

        Ok(())
    }

    fn parse_primary(&mut self) -> Result<(), Box<dyn Error>> {
        let tok = self.token_iterator.next().ok_or("")?;

        match tok.kind {
            TokenKind::Identifier(ident) => self.operand_stack.push(Node::Value(ValueNode::Identifier(ident))),
            TokenKind::Int(int) => self.operand_stack.push(Node::Value(ValueNode::Int(int))),
            TokenKind::Float(float) => self.operand_stack.push(Node::Value(ValueNode::Float(float))),
            TokenKind::Bool(bool) => self.operand_stack.push(Node::Value(ValueNode::Bool(bool))),
            TokenKind::String(string) => self.operand_stack.push(Node::Value(ValueNode::String(string))),

            TokenKind::LParen => {
                self.operator_stack.push(Operator::Sentinel);
                self.parse_expr()?;
                if !matches!(self.token_iterator.peek().ok_or("Error")?.kind, TokenKind::RParen) { todo!("Error") }
                self.operand_stack.pop();
            }

            _ if tok.to_unary().is_some() => {
                self.push_operator(Operator::Unary(tok.to_unary().unwrap()));
                self.parse_primary()?;
            }

            _ => { todo!("Error") }
        }

        Ok(())
    }

    fn pop_operator(&mut self) {
        match self.operator_stack.pop().unwrap() {
            Operator::Binary(binary_op) => {
                let t1 = self.operand_stack.pop().unwrap();
                let t0 = self.operand_stack.pop().unwrap();
                self.operand_stack.push(Node::Binary(BinaryNode::new(binary_op, t0, t1)));
            }
            Operator::Unary(unary_op) => {
                let t = self.operand_stack.pop().unwrap();
                self.operand_stack.push(Node::Unary(UnaryNode::new(unary_op, t)));
            }
            _ => unreachable!(),
        }
    }

    fn push_operator(&mut self, operator: Operator) {
        while self.operator_stack.last().unwrap().precedence() > operator.precedence() { self.pop_operator(); }
        self.operator_stack.push(operator);
    }
}
