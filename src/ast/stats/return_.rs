use crate::ast::node::Node;
use crate::ast::Exp;

#[derive(Clone, Debug)]
pub struct Return {
    pub exps: Vec<Node<Exp>>,
}

impl Return {
    pub fn new(exps: Vec<Node<Exp>>) -> Self {
        Self { exps }
    }
}
