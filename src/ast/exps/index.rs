use crate::ast::Exp;
use crate::ast::node::Node;

#[derive(Clone, Debug)]
pub struct Index {
    pub lhs: Box<Node<Exp>>,
    pub exp: Box<Node<Exp>>
}

impl Index {
    pub fn new(lhs: Node<Exp>, exp: Node<Exp>) -> Self {
        Self {
            lhs: Box::new(lhs),
            exp: Box::new(exp)
        }
    }
}

impl Into<Exp> for Node<Index> {
    fn into(self) -> Exp {
        Exp::Index(self)
    }
}
