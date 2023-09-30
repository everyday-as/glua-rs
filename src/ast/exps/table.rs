use crate::ast::{node::Node, Exp};

#[derive(Clone, Copy, Debug)]
pub struct TableConstructor<'a> {
    pub fields: &'a [Field<'a>],
}

#[derive(Clone, Copy, Debug)]
pub struct Field<'a> {
    pub key: Option<Node<&'a Exp<'a>>>,
    pub value: Node<&'a Exp<'a>>,
}

impl<'a> TableConstructor<'a> {
    pub fn new(fields: &'a [Field]) -> Self {
        Self { fields }
    }
}

impl<'a> Field<'a> {
    pub fn new(key: Option<Node<&'a Exp>>, value: Node<&'a Exp>) -> Self {
        Self { key, value }
    }
}
