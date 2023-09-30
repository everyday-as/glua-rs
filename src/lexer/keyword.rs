use crate::lexer::Token;

#[derive(Clone, Copy, Debug, PartialEq)]
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
    Goto,
}

impl PartialEq<Token<'_>> for Keyword {
    fn eq(&self, other: &Token) -> bool {
        match other {
            Token::Keyword(keyword) => self.eq(keyword),
            _ => false,
        }
    }
}
