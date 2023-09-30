use crate::ast::{node::Node, Block, Exp};

#[derive(Clone, Copy, Debug)]
pub struct For<'a> {
    pub init: (&'a str, Node<&'a Exp<'a>>),
    pub test: Node<&'a Exp<'a>>,
    pub update: Option<Node<&'a Exp<'a>>>,
    pub body: Block<'a>,
}

impl<'a> For<'a> {
    pub fn new(
        init: (&'a str, Node<&'a Exp>),
        test: Node<&'a Exp>,
        update: Option<Node<&'a Exp>>,
        body: Block<'a>,
    ) -> Self {
        Self {
            init,
            test,
            update,
            body,
        }
    }
}
