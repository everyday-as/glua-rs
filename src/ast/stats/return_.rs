use crate::ast::{node::Node, Exp};

#[derive(Clone, Copy, Debug)]
pub struct Return<'a> {
    pub exps: &'a [Node<&'a Exp<'a>>],
}

impl<'a> Return<'a> {
    pub fn new(exps: &'a [Node<&'a Exp>]) -> Self {
        Self { exps }
    }
}
