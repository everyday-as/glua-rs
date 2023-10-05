use std::fmt::{Display, Formatter};

use crate::ast::{
    Exp,
    node::Node,
    visitors::{renderer::Renderer, walk_member_exp},
};

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

impl Display for Node<&Member<'_>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Renderer::fmt(self, f, walk_member_exp)
    }
}
