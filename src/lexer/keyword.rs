use crate::lexer::Token;

#[derive(Clone, Debug, PartialEq)]
pub enum Keyword {
    Break,
    Do,
    Else,
    ElseIf,
    End,
    For,
    Function,
    If,
    In,
    Local,
    Repeat,
    Return,
    Then,
    Until,
    While,
    // GMod specific
    Continue,
}

impl Into<Token> for Keyword {
    fn into(self) -> Token {
        Token::Keyword(self)
    }
}
