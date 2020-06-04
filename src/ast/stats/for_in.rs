use crate::ast::{Stat, Exp, Block};

#[derive(Clone, Debug)]
pub struct ForIn {
    pub names: Vec<String>,
    pub exps: Vec<Exp>,
    pub body: Block
}

impl ForIn {
    pub fn new(names: Vec<String>, exps: Vec<Exp>, body: Block) -> Self {
        Self {
            names,
            exps,
            body
        }
    }
}

impl Into<Stat> for ForIn {
    fn into(self) -> Stat {
        Stat::ForIn(self)
    }
}

