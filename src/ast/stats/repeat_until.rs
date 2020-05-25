use crate::ast::{Exp, Stat, Block};

#[derive(Debug)]
pub struct RepeatUntil {
    body: Block,
    cond: Exp,
}

impl RepeatUntil {
    pub fn new(body: Block, cond: Exp) -> Self {
        Self {
            body,
            cond
        }
    }
}

impl Into<Stat> for RepeatUntil {
    fn into(self) -> Stat {
        Stat::RepeatUntil(self)
    }
}