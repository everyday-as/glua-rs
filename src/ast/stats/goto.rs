use crate::ast::node::Node;
use crate::ast::Stat;

#[derive(Clone, Debug)]
pub struct Goto {
    pub label: String,
}

impl Goto {
    pub fn new(label: String) -> Self {
        Self { label }
    }
}

impl Into<Stat> for Goto {
    fn into(self) -> Stat {
        Stat::Goto(self)
    }
}
