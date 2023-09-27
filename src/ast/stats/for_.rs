use crate::ast::node::Node;
use crate::ast::Block;
use crate::ast::Exp;

#[derive(Clone, Debug)]
pub struct For {
    pub init: (String, Node<Exp>),
    pub test: Node<Exp>,
    pub update: Option<Node<Exp>>,
    pub body: Block,
}

impl For {
    pub fn new(
        init: (String, Node<Exp>),
        test: Node<Exp>,
        update: Option<Node<Exp>>,
        body: Block,
    ) -> Self {
        Self {
            init,
            test,
            update,
            body,
        }
    }
}
