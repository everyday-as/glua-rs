use crate::ast::node::Node;
use crate::ast::Exp;

#[derive(Clone, Debug)]
pub struct MethodCall {
    pub lhs: Box<Node<Exp>>,
    pub name: String,
    pub args: Vec<Node<Exp>>,
}

impl MethodCall {
    pub fn new(lhs: Node<Exp>, name: String, args: Vec<Node<Exp>>) -> Self {
        Self {
            lhs: Box::new(lhs),
            name,
            args,
        }
    }
}

impl Into<Exp> for MethodCall {
    fn into(self) -> Exp {
        Exp::MethodCall(self)
    }
}
