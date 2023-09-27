use crate::ast::node::Node;
use crate::ast::Exp;

#[derive(Clone, Debug)]
pub struct VarDef {
    pub names: Vec<String>,
    pub init_exps: Option<Vec<Node<Exp>>>,
}

impl VarDef {
    pub fn new(names: Vec<String>, init_exps: Option<Vec<Node<Exp>>>) -> Self {
        Self { names, init_exps }
    }
}
