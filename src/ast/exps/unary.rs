use std::{
    fmt,
    fmt::{Display, Formatter},
};

use crate::ast::{node::Node, Exp};

#[derive(Clone, Copy, Debug)]
pub struct Unary<'a> {
    pub op: UnOp,
    pub exp: Node<&'a Exp<'a>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnOp {
    Neg,
    Not,
    Len,
}

impl<'a> Unary<'a> {
    pub fn new(op: UnOp, exp: Node<&'a Exp>) -> Self {
        Self { op, exp }
    }
}

impl Display for UnOp {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                UnOp::Neg => "-",
                UnOp::Not => "not",
                UnOp::Len => "#",
            }
        )
    }
}
