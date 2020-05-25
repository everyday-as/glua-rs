use crate::lexer::{Keyword, Literal, Op};

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Comma,
    Comment(String),
    Ellipsis,
    Keyword(Keyword),
    LBrace,
    LBracket,
    Literal(Literal),
    LParens,
    Name(String),
    Op(Op),
    RBrace,
    RBracket,
    RParens,
    Semicolon,
    Whitespace(usize)
}