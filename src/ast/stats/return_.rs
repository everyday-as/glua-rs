use crate::ast::node::Node;
use crate::ast::{Exp, Stat};

#[derive(Clone, Debug)]
pub struct Return {
    pub exps: Vec<Node<Exp>>,
}

impl Return {
    pub fn new(exps: Vec<Node<Exp>>) -> Self {
        Self { exps }
    }
}

impl Into<Stat> for Return {
    fn into(self) -> Stat {
        Stat::Return(self)
    }
}
