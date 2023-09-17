use crate::ast::node::Node;
use crate::ast::Stat;

#[derive(Clone, Debug, PartialEq)]
pub struct Label {
    name: String,
}

impl Label {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Into<Stat> for Node<Label> {
    fn into(self) -> Stat {
        Stat::Label(self)
    }
}
