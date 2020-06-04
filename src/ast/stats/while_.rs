use crate::ast::{Exp, Stat, Block};

#[derive(Clone, Debug)]
pub struct While {
    pub body: Block,
    pub cond: Exp
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