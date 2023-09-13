use crate::ast::node::Node;
use crate::ast::{Block, Stat};

#[derive(Clone, Debug)]
pub struct Do {
    pub body: Block,
}

impl Do {
    pub fn new(body: Block) -> Self {
        Self { body }
    }
}

impl Into<Stat> for Node<Do> {
    fn into(self) -> Stat {
        Stat::Do(self)
    }
}
