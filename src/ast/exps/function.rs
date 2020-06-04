use crate::ast::{Block, Exp};

#[derive(Clone, Debug)]
pub struct Function {
    pub params: Vec<String>,
    pub body: Block,
}

impl Function {
    pub fn new(params: Vec<String>, body: Block) -> Self {
        Self {
            params,
            body,
        }
    }
}

impl Into<Exp> for Function {
    fn into(self) -> Exp {
        Exp::Function(self)
    }
}
