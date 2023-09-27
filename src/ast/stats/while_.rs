use crate::ast::node::Node;
use crate::ast::{Block, Exp, Stat};

#[derive(Clone, Debug)]
pub struct While {
    pub body: Block,
    pub cond: Node<Exp>,
}

impl While {
    pub fn new(cond: Node<Exp>, body: Block) -> Self {
        Self { body, cond }
    }
}

impl Into<Stat> for While {
    fn into(self) -> Stat {
        Stat::While(self)
    }
}
