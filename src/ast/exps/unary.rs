use std::fmt::{Display, Formatter};
use std::fmt;

use crate::ast::Exp;

#[derive(Clone, Debug)]
pub struct Unary {
    pub op: UnOp,
    pub exp: Box<Exp>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UnOp {
    Neg,
    Not,
    Len,
}

impl Unary {
    pub fn new(op: UnOp, exp: Exp) -> Self {
        Self {
            op,
            exp: Box::new(exp),
        }
    }
}

impl Into<Exp> for Unary {
    fn into(self) -> Exp {
        Exp::Unary(self)
    }
}

impl Display for UnOp {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            UnOp::Neg => "-",
            UnOp::Not => "not",
            UnOp::Len => "#"
        })
    }
}
