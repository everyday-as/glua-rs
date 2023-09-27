use crate::ast::node::Node;
use crate::ast::Exp;

#[derive(Clone, Debug)]
pub struct TableConstructor {
    pub fields: Vec<Field>,
}

#[derive(Clone, Debug)]
pub struct Field {
    pub key: Option<Box<Node<Exp>>>,
    pub value: Box<Node<Exp>>,
}

impl TableConstructor {
    pub fn new(fields: Vec<Field>) -> Self {
        Self { fields }
    }
}

impl Field {
    pub fn new(key: Option<Node<Exp>>, value: Node<Exp>) -> Self {
        Self {
            key: key.map(Box::new),
            value: Box::new(value),
        }
    }
}
