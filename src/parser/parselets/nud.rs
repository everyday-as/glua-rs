use bumpalo::collections::Vec;

use crate::{
    ast::{
        exps::{table::Field, unary::UnOp, TableConstructor, Unary},
        Exp,
    },
    lexer::{Keyword, Literal, Op, Token},
    parser::{parselets::Nud, Parser, Precedence, Result},
};

pub struct EllipsisParselet;

impl Nud for EllipsisParselet {
    fn parse<'a>(&self, _parser: &mut Parser<'a>, token: Token<'a>) -> Result<'a, Exp<'a>> {
        assert_eq!(Token::Ellipsis, token);

        Ok(Exp::VarArgs)
    }
}

pub struct FunctionParselet;

impl Nud for FunctionParselet {
    fn parse<'a>(&self, parser: &mut Parser<'a>, token: Token<'a>) -> Result<'a, Exp<'a>> {
        assert_eq!(Token::Keyword(Keyword::Function), token);

        parser.parse_function().map(|function| function.into())
    }
}

pub struct LiteralParselet;

impl Nud for LiteralParselet {
    fn parse<'a>(&self, _parser: &mut Parser<'a>, token: Token<'a>) -> Result<'a, Exp<'a>> {
        match token {
            Token::Literal(literal) => match literal {
                Literal::Bool(value) => Ok(Exp::Bool(value)),
                Literal::Nil => Ok(Exp::Nil),
                Literal::Number(value) => Ok(Exp::Number(value)),
                Literal::String(value) => Ok(Exp::String(value)),
            },

            _ => unreachable!(),
        }
    }
}

pub struct NameParselet;

impl Nud for NameParselet {
    fn parse<'a>(&self, _parser: &mut Parser<'a>, token: Token<'a>) -> Result<'a, Exp<'a>> {
        let name = match token {
            Token::Name(name) => name,
            Token::Keyword(Keyword::Goto) => "goto",
            _ => unreachable!(),
        };

        Ok(Exp::Ref(name))
    }
}

pub struct ParensParselet;

impl Nud for ParensParselet {
    fn parse<'a>(&self, parser: &mut Parser<'a>, token: Token<'a>) -> Result<'a, Exp<'a>> {
        assert_eq!(Token::LParens, token);

        let exp = parser.parse_exp()?;

        parser.expect(Token::RParens)?;

        Ok(exp)
    }
}

pub struct TableConstructorParselet;

impl Nud for TableConstructorParselet {
    fn parse<'a>(&self, parser: &mut Parser<'a>, token: Token<'a>) -> Result<'a, Exp<'a>> {
        assert_eq!(Token::LBrace, token);

        let mut fields = Vec::new_in(parser.bump);

        while !parser.consume_a(Token::RBrace) {
            fields.push(match parser.peek(0)? {
                // { name = Exp }
                Token::Keyword(Keyword::Goto) | Token::Name(_)
                    if parser.peek(1)? == &Token::Op(Op::Eq) =>
                {
                    let key = parser.node(|p| p.parse_name().map(Exp::String))?;

                    parser.consume()?;

                    let value = parser.node(Parser::parse_exp)?;

                    let field = Field::new(Some(key), value);

                    parser.consume_a(Token::Semicolon);
                    parser.consume_a(Token::Comma);

                    field
                }

                // { [Exp] = Exp }
                Token::LBracket => {
                    parser.consume()?;

                    let key = parser.node(Parser::parse_exp)?;

                    parser.expect(Token::RBracket)?;

                    parser.expect(Op::Eq)?;

                    let value = parser.node(Parser::parse_exp)?;

                    parser.consume_a(Token::Semicolon);
                    parser.consume_a(Token::Comma);

                    Field::new(Some(key), value)
                }

                // { Exp }
                _ => {
                    let value = parser.node(Parser::parse_exp)?;

                    parser.consume_a(Token::Semicolon);
                    parser.consume_a(Token::Comma);

                    Field::new(None, value)
                }
            })
        }

        Ok(TableConstructor::new(fields.into_bump_slice()).into())
    }
}

pub struct UnaryParselet;

impl Nud for UnaryParselet {
    fn parse<'a>(&self, parser: &mut Parser<'a>, token: Token<'a>) -> Result<'a, Exp<'a>> {
        let op = match token {
            Token::Op(Op::Len) => UnOp::Len,
            Token::Op(Op::Not) => UnOp::Not,
            Token::Op(Op::Sub) => UnOp::Neg,

            _ => unreachable!(),
        };

        let exp = parser.node(|p| p.parse_exp_prec(Precedence::Unary))?;

        Ok(Unary::new(op, exp).into())
    }
}
