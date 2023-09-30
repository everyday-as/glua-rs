use crate::ast::{node::Node, Block, Exp};

#[derive(Clone, Copy, Debug)]
pub struct RepeatUntil<'a> {
    pub body: Block<'a>,
    pub cond: Node<&'a Exp<'a>>,
}

impl<'a> RepeatUntil<'a> {
    pub fn new(body: Block<'a>, cond: Node<&'a Exp>) -> Self {
        Self { body, cond }
    }
}
