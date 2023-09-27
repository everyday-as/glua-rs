use crate::ast::node::Node;
use crate::ast::Exp;

#[derive(Clone, Debug)]
pub struct Index {
    pub lhs: Box<Node<Exp>>,
    pub exp: Box<Node<Exp>>,
}

impl Index {
    pub fn new(lhs: Node<Exp>, exp: Node<Exp>) -> Self {
        Self {
            lhs: Box::new(lhs),
            exp: Box::new(exp),
        }
    }
}

impl Into<Exp> for Index {
    fn into(self) -> Exp {
        Exp::Index(self)
    }
}
