use crate::ast::node::Node;
use crate::ast::{Exp, Stat};

#[derive(Clone, Debug)]
pub struct Assignment {
    pub vars: Vec<Node<Exp>>,
    pub exps: Vec<Node<Exp>>,
}

impl Assignment {
    pub fn new(vars: Vec<Node<Exp>>, exps: Vec<Node<Exp>>) -> Self {
        Self { vars, exps }
    }
}

impl Into<Stat> for Node<Assignment> {
    fn into(self) -> Stat {
        Stat::Assignment(self)
    }
}
