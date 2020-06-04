use crate::ast::{Exp, Stat, Block};

#[derive(Clone, Debug)]
pub struct IfElse {
    pub cond: Exp,
    pub body: Block,
    pub else_ifs: Vec<(Exp, Block)>,
    pub else_block: Option<Block>
}

impl IfElse {
    pub fn new(cond: Exp, body: Block, else_ifs: Vec<(Exp, Block)>, else_block: Option<Block>) -> Self {
        Self {
            cond,
            body,
            else_ifs,
            else_block
        }
    }
}

impl Into<Stat> for IfElse {
    fn into(self) -> Stat {
        Stat::IfElse(self)
    }
}