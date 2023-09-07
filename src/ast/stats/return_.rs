use crate::ast::{Exp, Stat};
use crate::ast::node::Node;

#[derive(Clone, Debug)]
pub struct Return {
    pub exps: Vec<Node<Exp>>
}

impl Return {
    pub fn new(exps: Vec<Node<Exp>>) -> Self {
        Self {
            exps
        }
    }
}

impl Into<Stat> for Node<Return> {
    fn into(self) -> Stat {
        Stat::Return(self)
    }
}
