use crate::ast::Exp;

#[derive(Debug)]
pub struct MethodCall {
    lhs: Box<Exp>,
    name: String,
    args: Vec<Exp>
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
