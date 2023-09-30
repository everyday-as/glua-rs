use crate::ast::{node::Node, Block, Exp};

#[derive(Clone, Copy, Debug)]
pub struct IfElse<'a> {
    pub cond: Node<&'a Exp<'a>>,
    pub body: Block<'a>,
    pub else_ifs: &'a [(Node<&'a Exp<'a>>, Block<'a>)],
    pub else_block: Option<Block<'a>>,
}

impl<'a> IfElse<'a> {
    pub fn new(
        cond: Node<&'a Exp>,
        body: Block<'a>,
        else_ifs: &'a [(Node<&'a Exp>, Block<'a>)],
        else_block: Option<Block<'a>>,
    ) -> Self {
        Self {
            cond,
            body,
            else_ifs,
            else_block,
        }
    }
}
