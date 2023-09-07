use crate::ast::{Exp, Stat, Block};
use crate::ast::node::Node;

#[derive(Clone, Debug)]
pub struct RepeatUntil {
    pub body: Block,
    pub cond: Node<Exp>,
}

impl RepeatUntil {
    pub fn new(body: Block, cond: Node<Exp>) -> Self {
        Self {
            body,
            cond
        }
    }
}

impl Into<Stat> for Node<RepeatUntil> {
    fn into(self) -> Stat {
        Stat::RepeatUntil(self)
    }
}