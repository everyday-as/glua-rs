use crate::ast::{node::Node, Block, Exp};

#[derive(Clone, Copy, Debug)]
pub struct While<'a> {
    pub body: Block<'a>,
    pub cond: Node<&'a Exp<'a>>,
}

impl<'a> While<'a> {
    pub fn new(cond: Node<&'a Exp>, body: Block<'a>) -> Self {
        Self { body, cond }
    }
}
