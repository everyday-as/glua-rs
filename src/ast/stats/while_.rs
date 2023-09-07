use crate::ast::{Exp, Stat, Block};
use crate::ast::node::Node;

#[derive(Clone, Debug)]
pub struct While {
    pub body: Block,
    pub cond: Node<Exp>
}

impl While {
    pub fn new(cond: Node<Exp>, body: Block) -> Self {
        Self {
            body,
            cond
        }
    }
}

impl Into<Stat> for Node<While> {
    fn into(self) -> Stat {
        Stat::While(self)
    }
}