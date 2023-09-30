use crate::lexer::Token;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Op {
    Add,
    And,
    Colon,
    Div,
    Dot,
    DotDot,
    Eq,
    EqEq,
    Exp,
    Gt,
    GtEq,
    Len,
    Lt,
    LtEq,
    Mod,
    Mul,
    Ne,
    Or,
    Not,
    Sub,
}

impl PartialEq<Token<'_>> for Op {
    fn eq(&self, other: &Token) -> bool {
        match other {
            Token::Op(op) => self.eq(op),
            _ => false,
        }
    }
}
