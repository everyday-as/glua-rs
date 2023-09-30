use crate::ast::{node::Node, Block, Exp};

#[derive(Clone, Copy, Debug)]
pub struct ForIn<'a> {
    pub names: &'a [&'a str],
    pub exps: &'a [Node<&'a Exp<'a>>],
    pub body: Block<'a>,
}

impl<'a> ForIn<'a> {
    pub fn new(names: &'a [&'a str], exps: &'a [Node<&'a Exp>], body: Block<'a>) -> Self {
        Self { names, exps, body }
    }
}
