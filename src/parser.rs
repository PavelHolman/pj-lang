use std::error::Error;
use crate::lexer::LexIter;
use crate::node::Node;

#[derive(Debug)]
pub struct AST {
    nodes: Vec<Node>,
}

struct Parser {}

impl Parser {
    fn new(lexer: LexIter) -> Self {
        todo!()
    }

    fn parse_node(&mut self) -> Node {
        todo!()
    }
}

pub fn parse(source_code: &str) -> Result<AST, Box<dyn Error>> {
    let mut nodes = Vec::<Node>::new();

    let lexer = LexIter::new(&source_code);
    let mut parser = Parser::new(lexer);

    while let node = parser.parse_node() { nodes.push(node); }

    Ok(AST { nodes })
}
