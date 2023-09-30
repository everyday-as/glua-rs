use crate::ast::{node::Node, Exp};

#[derive(Clone, Copy, Debug)]
pub struct Index<'a> {
    pub lhs: Node<&'a Exp<'a>>,
    pub exp: Node<&'a Exp<'a>>,
}

impl<'a> Index<'a> {
    pub fn new(lhs: Node<&'a Exp>, exp: Node<&'a Exp>) -> Self {
        Self { lhs, exp }
    }
}
