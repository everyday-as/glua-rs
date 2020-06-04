use crate::ast::Exp;

#[derive(Clone, Debug)]
pub struct FunctionCall {
    pub lhs: Box<Exp>,
    pub args: Vec<Exp>
}

impl FunctionCall {
    pub fn new(lhs: Exp, args: Vec<Exp>) -> Self {
        Self {
            lhs: Box::new(lhs),
            args
        }
    }
}

impl Into<Exp> for FunctionCall {
    fn into(self) -> Exp {
        Exp::FunctionCall(self)
    }
}
