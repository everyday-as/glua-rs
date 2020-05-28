use crate::ast::{Block, Exp};

#[derive(Debug)]
pub struct Function {
    params: Vec<String>,
    body: Block,
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
