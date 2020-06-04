use std::fmt::{Display, Formatter};

use crate::ast::Exp;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Binary {
    pub lhs: Box<Exp>,
    pub op: BinOp,
    pub rhs: Box<Exp>
}

#[derive(Clone, Debug, Eq, PartialEq)]
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

impl Display for BinOp {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            BinOp::Add => "+",
            BinOp::And => "and",
            BinOp::Concat => "..",
            BinOp::Div => "/",
            BinOp::Eq => "==",
            BinOp::Exp => "^",
            BinOp::Gt => ">",
            BinOp::GtEq => ">=",
            BinOp::Lt => "<",
            BinOp::LtEq => "<=",
            BinOp::Mod => "%",
            BinOp::Mul => "*",
            BinOp::Ne => "~=",
            BinOp::Or => "or",
            BinOp::Sub => "-",
        })
    }
}
