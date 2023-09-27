use crate::ast::node::Node;
use crate::ast::{Block, Exp, Stat};

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

impl Into<Stat> for ForIn {
    fn into(self) -> Stat {
        Stat::ForIn(self)
    }
}
