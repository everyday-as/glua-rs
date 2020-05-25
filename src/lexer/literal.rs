use crate::lexer::Token;

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Bool(bool),
    Nil,
    Number(f64),
    String(String)
}

impl Into<Token> for Literal {
    fn into(self) -> Token {
        Token::Literal(self)
    }
}