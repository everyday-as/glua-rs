use crate::ast::node::Node;
use crate::ast::{Block, Exp, Stat};

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

impl Into<Stat> for RepeatUntil {
    fn into(self) -> Stat {
        Stat::RepeatUntil(self)
    }
}
