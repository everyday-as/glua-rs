use crate::lexer::Token;
use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq)]
pub enum Literal<'a> {
    Bool(bool),
    Nil,
    Number(f64),
    String(Cow<'a, str>),
}

impl<'a> Into<Token<'a>> for Literal<'a> {
    fn into(self) -> Token<'a> {
        Token::Literal(self)
    }
}
