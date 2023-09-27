use crate::ast::node::Node;
use crate::ast::Block;
use crate::ast::Exp;

#[derive(Clone, Debug)]
pub struct ForIn {
    pub names: Vec<String>,
    pub exps: Vec<Node<Exp>>,
    pub body: Block,
}

impl ForIn {
    pub fn new(names: Vec<String>, exps: Vec<Node<Exp>>, body: Block) -> Self {
        Self { names, exps, body }
    }
}
