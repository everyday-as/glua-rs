use crate::ast::node::Node;
use crate::ast::Exp;

#[derive(Clone, Debug)]
pub struct FunctionCall {
    pub lhs: Box<Node<Exp>>,
    pub args: Vec<Node<Exp>>,
}

impl FunctionCall {
    pub fn new(lhs: Node<Exp>, args: Vec<Node<Exp>>) -> Self {
        Self {
            lhs: Box::new(lhs),
            args,
        }
    }
}

impl Into<Exp> for FunctionCall {
    fn into(self) -> Exp {
        Exp::FunctionCall(self)
    }
}
