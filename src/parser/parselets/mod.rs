use crate::ast::Exp;
use crate::lexer::Token;
use crate::parser::{Parser, Precedence};

pub mod nud;
pub mod led;

// Null-denotation rule
pub trait Nud {
    fn parse(&self, parser: &mut Parser, token: Token) -> Result<Exp, String>;
}

// Left-denotation rule
pub trait Led {
    fn parse(&self, parser: &mut Parser, lhs: Exp, token: Token) -> Result<Exp, String>;
    fn get_precedence(&self) -> Precedence;
}
