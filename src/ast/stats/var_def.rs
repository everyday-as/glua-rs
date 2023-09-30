use crate::ast::{node::Node, Exp};

#[derive(Clone, Copy, Debug)]
pub struct VarDef<'a> {
    pub names: &'a [&'a str],
    pub init_exps: Option<&'a [Node<&'a Exp<'a>>]>,
}

impl<'a> VarDef<'a> {
    pub fn new(names: &'a [&'a str], init_exps: Option<&'a [Node<&'a Exp>]>) -> Self {
        Self { names, init_exps }
    }
}
