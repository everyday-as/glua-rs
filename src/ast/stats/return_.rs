use crate::ast::{Exp, Stat};

#[derive(Clone, Debug)]
pub struct Return {
    pub exps: Vec<Exp>
}

impl Return {
    pub fn new(exps: Vec<Exp>) -> Self {
        Self {
            exps
        }
    }
}

impl Into<Stat> for Return {
    fn into(self) -> Stat {
        Stat::Return(self)
    }
}
