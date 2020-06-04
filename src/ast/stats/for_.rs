use crate::ast::{Stat, Exp, Block};
use crate::ast::stats::VarDef;

#[derive(Clone, Debug)]
pub struct For {
    pub init: (String, Exp),
    pub test: Exp,
    pub update: Option<Exp>,
    pub body: Block
}

impl For {
    pub fn new(init: (String, Exp), test: Exp, update: Option<Exp>, body: Block) -> Self {
        Self {
            init,
            test,
            update,
            body
        }
    }
}

impl Into<Stat> for For {
    fn into(self) -> Stat {
        Stat::For(self)
    }
}