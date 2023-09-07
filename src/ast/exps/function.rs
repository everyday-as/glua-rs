use crate::ast::{Block, Exp};
use crate::ast::node::Node;

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

impl Into<Exp> for Node<Function> {
    fn into(self) -> Exp {
        Exp::Function(self)
    }
}
