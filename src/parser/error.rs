use std::fmt::{Display, Formatter};

use logos::Span;

use crate::{
    ast::Exp,
    lexer::{Keyword, Op, Token},
};

#[derive(Debug)]
pub enum Expectation<'a> {
    Args,
    Eof,
    Expression,
    FunctionCall,
    Keyword(Keyword),
    Name,
    Op(Op),
    PrefixExp,
    Stat,
    Token(Token<'a>),
    Tokens(Vec<Token<'a>>),
    Var,
}

#[derive(thiserror::Error, Debug)]
pub enum Error<'a> {
    UnexpectedEof {
        expected: Option<Expectation<'a>>,
    },
    UnexpectedExp {
        span: Span,
        expected: Expectation<'a>,
        got: Exp<'a>,
    },
    UnexpectedToken {
        span: Span,
        expected: Option<Expectation<'a>>,
        got: Token<'a>,
    },
    Lexer(Span),
}

impl<'a> Expectation<'a> {
    pub fn tokens<T>(tokens: impl IntoIterator<Item = T>) -> Self
    where
        T: Into<Token<'a>>,
    {
        Self::Tokens(tokens.into_iter().map(|t| t.into()).collect())
    }
}

impl Display for Expectation<'_> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Args => write!(f, "args"),
            Self::Eof => write!(f, "eof"),
            Self::Expression => write!(f, "expression"),
            Self::FunctionCall => write!(f, "functioncall"),
            Self::Keyword(keyword) => write!(f, "{:?}", keyword),
            Self::Name => write!(f, "name"),
            Self::Op(op) => write!(f, "{:?}", op),
            Self::PrefixExp => write!(f, "prefixexp"),
            Self::Stat => write!(f, "stat"),
            Self::Token(token) => write!(f, "{:?}", token),
            Self::Tokens(tokens) => match tokens.len() {
                1 => write!(f, "{:?}", tokens[0]),
                2 => write!(f, "{:?} or {:?}", tokens[0], tokens[1]),
                _ => {
                    for token in &tokens[..tokens.len() - 2] {
                        write!(f, "{:?}, ", token)?;
                    }

                    write!(
                        f,
                        "{:?} or {:?}",
                        tokens[tokens.len() - 2],
                        tokens[tokens.len() - 1]
                    )
                }
            },
            Self::Var => write!(f, "var"),
        }
    }
}

impl<'a> From<Token<'a>> for Option<Expectation<'a>> {
    fn from(value: Token<'a>) -> Self {
        Some(Expectation::Token(value))
    }
}

impl From<Keyword> for Option<Expectation<'_>> {
    fn from(value: Keyword) -> Self {
        Some(Expectation::Keyword(value))
    }
}

impl From<Op> for Option<Expectation<'_>> {
    fn from(value: Op) -> Self {
        Some(Expectation::Op(value))
    }
}

impl<'a, T> From<T> for Expectation<'a>
where
    T: Into<Token<'a>>,
{
    fn from(value: T) -> Self {
        Self::Token(value.into())
    }
}

impl<'a> Error<'a> {
    pub(crate) fn unexpected_eof(expected: impl Into<Option<Expectation<'a>>>) -> Self {
        Self::UnexpectedEof {
            expected: expected.into(),
        }
    }

    pub(crate) fn unexpected_expression(
        span: &Span,
        expected: impl Into<Expectation<'a>>,
        got: Exp<'a>,
    ) -> Self {
        Self::UnexpectedExp {
            span: span.to_owned(),
            expected: expected.into(),
            got,
        }
    }

    pub(crate) fn unexpected_token(
        span: &Span,
        expected: impl Into<Option<Expectation<'a>>>,
        got: impl Into<Token<'a>>,
    ) -> Self {
        Self::UnexpectedToken {
            span: span.to_owned(),
            expected: expected.into(),
            got: got.into(),
        }
    }
}

impl Display for Error<'_> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::UnexpectedToken {
                span,
                expected,
                got,
            } => {
                write!(f, "Unexpected {:?}", got)?;

                if let Some(expectation) = expected {
                    write!(f, ", expecting {}", expectation)?;
                }

                write!(f, " at {:?}", span)
            }
            Self::UnexpectedEof { expected } => {
                write!(f, "Unexpected EOF")?;

                if let Some(expectation) = expected {
                    write!(f, ", expecting {}", expectation)?;
                }

                Ok(())
            }
            Self::UnexpectedExp {
                span,
                expected,
                got,
            } => write!(
                f,
                "Unexpected {:?}, expecting {} at {:?}",
                got, expected, span
            ),
            Self::Lexer(span) => write!(f, "Unrecognised token at `{:?}` in input", span),
        }
    }
}
