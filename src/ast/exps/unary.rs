use crate::ast::Exp;

#[derive(Debug)]
pub struct Unary {
    op: UnOp,
    exp: Box<Exp>
}

#[derive(Debug)]
pub enum UnOp {
    Neg,
    Not,
    Len
}

impl Unary {
    pub fn new(op: UnOp, exp: Exp) -> Self {
        Self {
            op,
            exp: Box::new(exp)
        }
    }
}

impl Into<Exp> for Unary {
    fn into(self) -> Exp {
        Exp::Unary(self)
    }
}
