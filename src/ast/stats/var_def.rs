use crate::ast::{Exp, Stat};

#[derive(Clone, Debug)]
pub struct VarDef {
    pub names: Vec<String>,
    pub init_exps: Option<Vec<Exp>>
}

impl VarDef {
    pub fn new(names: Vec<String>, init_exps: Option<Vec<Exp>>) -> Self {
        Self {
            names,
            init_exps,
        }
    }
}

impl Into<Stat> for VarDef {
    fn into(self) -> Stat {
        Stat::VarDef(self)
    }
}