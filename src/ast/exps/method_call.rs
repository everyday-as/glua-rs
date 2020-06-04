use crate::ast::Exp;

#[derive(Clone, Debug)]
pub struct MethodCall {
    pub lhs: Box<Exp>,
    pub name: String,
    pub args: Vec<Exp>
}

impl MethodCall {
    pub fn new(lhs: Exp, name: String, args: Vec<Exp>) -> Self {
        Self {
            lhs: Box::new(lhs),
            name,
            args
        }
    }
}

impl Into<Exp> for MethodCall {
    fn into(self) -> Exp {
        Exp::MethodCall(self)
    }
}
