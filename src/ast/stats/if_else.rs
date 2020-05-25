use crate::ast::{Exp, Stat, Block};

#[derive(Debug)]
pub struct IfElse {
    cond: Exp,
    else_ifs: Vec<(Exp, Block)>,
    else_block: Option<Block>
}

impl IfElse {
    pub fn new(cond: Exp, else_ifs: Vec<(Exp, Block)>, else_block: Option<Block>) -> Self {
        Self {
            cond,
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