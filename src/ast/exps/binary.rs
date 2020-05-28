use crate::ast::Exp;

#[derive(Debug)]
pub struct Binary {
    lhs: Box<Exp>,
    op: BinOp,
    rhs: Box<Exp>
}

#[derive(Debug)]
pub enum BinOp {
    Add,
    And,
    Concat,
    Div,
    Eq,
    Exp,
    Gt,
    GtEq,
    Lt,
    LtEq,
    Mod,
    Mul,
    Ne,
    Or,
    Sub,
}

impl Binary {
    pub fn new(lhs: Exp, op: BinOp, rhs: Exp) -> Self {
        Self {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs)
        }
    }
}

impl Into<Exp> for Binary {
    fn into(self) -> Exp {
        Exp::Binary(self)
    }
}
