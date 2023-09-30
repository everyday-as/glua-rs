use crate::ast::{node::Node, Exp};

#[derive(Clone, Copy, Debug)]
pub struct Assignment<'a> {
    pub vars: &'a [Node<&'a Exp<'a>>],
    pub exps: &'a [Node<&'a Exp<'a>>],
}

impl<'a> Assignment<'a> {
    pub fn new(vars: &'a [Node<&'a Exp>], exps: &'a [Node<&'a Exp>]) -> Self {
        Self { vars, exps }
    }
}
