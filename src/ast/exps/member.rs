use crate::ast::node::Node;
use crate::ast::Exp;

#[derive(Clone, Debug)]
pub struct Member {
    pub lhs: Box<Node<Exp>>,
    pub name: String,
}

impl Member {
    pub fn new(lhs: Node<Exp>, name: String) -> Self {
        Self {
            lhs: Box::new(lhs),
            name,
        }
    }
}

impl Into<Exp> for Node<Member> {
    fn into(self) -> Exp {
        Exp::Member(self)
    }
}
