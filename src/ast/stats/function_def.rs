use crate::ast::exps::Function;
use crate::ast::Stat;

#[derive(Debug)]
pub struct FunctionDef {
    local: bool,
    name: String,
    body: Function,
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
