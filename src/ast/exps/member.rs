use crate::ast::{node::Node, Exp};

#[derive(Clone, Copy, Debug)]
pub struct Member<'a> {
    pub lhs: Node<&'a Exp<'a>>,
    pub name: &'a str,
}

impl<'a> Member<'a> {
    pub fn new(lhs: Node<&'a Exp>, name: &'a str) -> Self {
        Self { lhs, name }
    }
}
