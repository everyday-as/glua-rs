use crate::ast::exps::Function;
use crate::ast::Stat;

#[derive(Clone, Debug)]
pub struct FunctionDef {
    pub local: bool,
    pub name: String,
    pub body: Function,
}

impl FunctionDef {
    pub fn new(local: bool, name: String, body: Function) -> Self {
        Self {
            local,
            name,
            body,
        }
    }
}

impl Into<Stat> for FunctionDef {
    fn into(self) -> Stat {
        Stat::FunctionDef(self)
    }
}
