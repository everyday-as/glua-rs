use crate::ast::{Exp, Stat};

#[derive(Debug)]
pub struct Assignment {
    vars: Vec<Exp>,
    exps: Vec<Exp>,
}

impl Assignment {
    pub fn new(vars: Vec<Exp>, exps: Vec<Exp>) -> Self {
        Self {
            vars,
            exps
        }
    }
}

impl Into<Stat> for Assignment {
    fn into(self) -> Stat {
        Stat::Assignment(self)
    }
}
