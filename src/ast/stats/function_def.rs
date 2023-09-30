use crate::ast::{exps::Function, node::Node};

#[derive(Clone, Copy, Debug)]
pub struct FunctionDef<'a> {
    pub local: bool,
    pub name: &'a str,
    pub body: Node<&'a Function<'a>>,
}

impl<'a> FunctionDef<'a> {
    pub fn new(local: bool, name: &'a str, body: Node<&'a Function>) -> Self {
        Self { local, name, body }
    }
}
