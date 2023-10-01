use crate::{
    ast::{node::Node, Exp},
    lexer::Token,
    parser::{Parser, Precedence, Result},
};

pub mod led;
pub mod nud;

// Null-denotation rule
pub trait Nud {
    fn parse<'a>(&self, parser: &mut Parser<'a>, token: &'a Token<'a>) -> Result<'a, Exp<'a>>;
}

// Left-denotation rule
pub trait Led {
    fn parse<'a>(
        &self,
        parser: &mut Parser<'a>,
        lhs: Node<&'a Exp>,
        token: &'a Token<'a>,
    ) -> Result<'a, Exp<'a>>;
    fn get_precedence(&self) -> Precedence;
}
