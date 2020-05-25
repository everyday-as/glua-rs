use crate::ast::{Exp, Stat};

#[derive(Debug)]
pub struct Assignment {
    vars: Vec<Exp>,
    exprs: Vec<Exp>,
}

impl Assignment {
    pub fn new(vars: Vec<Exp>, exprs: Vec<Exp>) -> Self {
        Self {
            vars,
            exprs,
        }
    }
}

impl Into<Stat> for Assignment {
    fn into(self) -> Stat {
        Stat::Assignment(self)
    }
}