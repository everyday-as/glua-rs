use crate::ast::Exp;
use crate::ast::node::Node;

#[derive(Clone, Debug)]
pub struct TableConstructor {
    pub fields: Vec<Field>
}

#[derive(Clone, Debug)]
pub struct Field {
    pub key: Option<Box<Node<Exp>>>,
    pub value: Box<Node<Exp>>,
}

impl TableConstructor {
    pub fn new(fields: Vec<Field>) -> Self {
        Self {
            fields
        }
    }
}

impl Into<Exp> for Node<TableConstructor> {
    fn into(self) -> Exp {
        Exp::Table(self)
    }
}

impl Field {
    pub fn new(key: Option<Node<Exp>>, value: Node<Exp>) -> Self {
        Self {
            key: key.map(|key| Box::new(key)),
            value: Box::new(value),
        }
    }
}
