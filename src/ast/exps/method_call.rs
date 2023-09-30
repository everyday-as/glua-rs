use crate::ast::{node::Node, Exp};

#[derive(Clone, Copy, Debug)]
pub struct MethodCall<'a> {
    pub lhs: Node<&'a Exp<'a>>,
    pub name: &'a str,
    pub args: &'a [Node<&'a Exp<'a>>],
}

impl<'a> MethodCall<'a> {
    pub fn new(lhs: Node<&'a Exp>, name: &'a str, args: &'a [Node<&'a Exp>]) -> Self {
        Self { lhs, name, args }
    }
}
