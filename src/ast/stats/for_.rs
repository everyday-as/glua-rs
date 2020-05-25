use crate::ast::{Stat, Exp, Block};
use crate::ast::stats::Assignment;

#[derive(Debug)]
pub struct For {
    init: Assignment,
    test: Exp,
    update: Option<Exp>,
    body: Block
}

impl For {
    pub fn new(init: Assignment, test: Exp, update: Option<Exp>, body: Block) -> Self {
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