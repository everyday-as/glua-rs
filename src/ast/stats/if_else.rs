use crate::ast::node::Node;
use crate::ast::{Block, Exp, Stat};

#[derive(Clone, Debug)]
pub struct IfElse {
    pub cond: Node<Exp>,
    pub body: Block,
    pub else_ifs: Vec<(Node<Exp>, Block)>,
    pub else_block: Option<Block>,
}

impl IfElse {
    pub fn new(
        cond: Node<Exp>,
        body: Block,
        else_ifs: Vec<(Node<Exp>, Block)>,
        else_block: Option<Block>,
    ) -> Self {
        Self {
            cond,
            body,
            else_ifs,
            else_block,
        }
    }
}

impl Into<Stat> for IfElse {
    fn into(self) -> Stat {
        Stat::IfElse(self)
    }
}
