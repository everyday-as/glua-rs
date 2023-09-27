use crate::ast::node::Node;
use crate::ast::Block;
use crate::ast::Exp;

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
