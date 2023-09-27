use crate::lexer::Token;

#[derive(Clone, Debug, PartialEq)]
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

impl Into<Token<'_>> for Op {
    fn into(self) -> Token<'static> {
        Token::Op(self)
    }
}
