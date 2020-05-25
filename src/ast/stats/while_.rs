use crate::ast::{Exp, Stat, Block};

#[derive(Debug)]
pub struct While {
    body: Block,
    cond: Exp
}

impl While {
    pub fn new(cond: Exp, body: Block) -> Self {
        Self {
            body,
            cond
        }
    }
}

impl Into<Stat> for While {
    fn into(self) -> Stat {
        Stat::While(self)
    }
}