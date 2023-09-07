use crate::ast::Exp;
use crate::ast::node::Node;

#[derive(Clone, Debug)]
pub struct MethodCall {
    pub lhs: Box<Node<Exp>>,
    pub name: String,
    pub args: Vec<Node<Exp>>
}

impl MethodCall {
    pub fn new(lhs: Node<Exp>, name: String, args: Vec<Node<Exp>>) -> Self {
        Self {
            lhs: Box::new(lhs),
            name,
            args
        }
    }
}

impl Into<Exp> for Node<MethodCall> {
    fn into(self) -> Exp {
        Exp::MethodCall(self)
    }
}
