use crate::ast::node::Node;
use crate::ast::Block;
use crate::ast::Exp;

#[derive(Clone, Debug)]
pub struct RepeatUntil {
    pub body: Block,
    pub cond: Node<Exp>,
}

impl RepeatUntil {
    pub fn new(body: Block, cond: Node<Exp>) -> Self {
        Self { body, cond }
    }
}
