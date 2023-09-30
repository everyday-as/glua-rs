use crate::ast::{node::Node, Exp};

#[derive(Clone, Copy, Debug)]
pub struct FunctionCall<'a> {
    pub lhs: Node<&'a Exp<'a>>,
    pub args: &'a [Node<&'a Exp<'a>>],
}

impl<'a> FunctionCall<'a> {
    pub fn new(lhs: Node<&'a Exp>, args: &'a [Node<&'a Exp>]) -> Self {
        Self { lhs, args }
    }
}
