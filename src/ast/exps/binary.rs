use std::{
    fmt,
    fmt::{Display, Formatter},
};

use crate::ast::{node::Node, Exp};

#[derive(Clone, Copy, Debug)]
pub struct Binary<'a> {
    pub lhs: Node<&'a Exp<'a>>,
    pub op: BinOp,
    pub rhs: Node<&'a Exp<'a>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

impl<'a> Binary<'a> {
    pub fn new(lhs: Node<&'a Exp>, op: BinOp, rhs: Node<&'a Exp>) -> Self {
        Self { lhs, op, rhs }
    }
}

impl Display for BinOp {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
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
            }
        )
    }
}
