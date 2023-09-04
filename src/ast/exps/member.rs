use crate::ast::Exp;
use crate::ast::node::Node;

#[derive(Clone, Debug)]
pub struct Member {
    pub lhs: Box<Node<Exp>>,
    pub name: String
}

impl Member {
    pub fn new(lhs: Node<Exp>, name: String) -> Self {
        Self {
            lhs: Box::new(lhs),
            name
        }
    }
}

impl Into<Exp> for Member {
    fn into(self) -> Exp {
        Exp::Member(self)
    }
}
