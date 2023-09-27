use crate::ast::exps::table::Field;
use crate::ast::exps::unary::UnOp;
use crate::ast::exps::{TableConstructor, Unary};
use crate::ast::Exp;
use crate::lexer::{Keyword, Literal, Op, Token};
use crate::parser::parselets::Nud;
use crate::parser::{Parser, Precedence};

pub struct EllipsisParselet;

impl Nud for EllipsisParselet {
    fn parse(&self, _parser: &mut Parser, token: Token) -> Result<Exp, String> {
        assert_eq!(Token::Ellipsis, token);

        Ok(Exp::VarArgs)
    }
}

pub struct FunctionParselet;

impl Nud for FunctionParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> Result<Exp, String> {
        assert_eq!(Token::Keyword(Keyword::Function), token);

        parser.parse_function().map(|function| function.into())
    }
}

pub struct LiteralParselet;

impl Nud for LiteralParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> Result<Exp, String> {
        match token {
            Token::Literal(literal) => match literal {
                Literal::Bool(value) => Ok(Exp::Bool(value)),
                Literal::Nil => Ok(Exp::Nil),
                Literal::Number(value) => Ok(Exp::Number(value)),
                Literal::String(value) => Ok(Exp::String(value.into_owned())),
            },

            _ => unreachable!(),
        }
    }
}

pub struct NameParselet;

impl Nud for NameParselet {
    fn parse(&self, _parser: &mut Parser, token: Token) -> Result<Exp, String> {
        let name = match token {
            Token::Name(name) => name,
            Token::Keyword(Keyword::Goto) => "goto",
            _ => unreachable!(),
        };

        Ok(Exp::Ref(name.to_owned()))
    }
}

pub struct ParensParselet;

impl Nud for ParensParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> Result<Exp, String> {
        assert_eq!(Token::LParens, token);

        let exp = parser.parse_exp()?;

        parser.expect(Token::RParens)?;

        Ok(exp.inner)
    }
}

pub struct TableConstructorParselet;

impl Nud for TableConstructorParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> Result<Exp, String> {
        assert_eq!(Token::LBrace, token);

        let mut fields = Vec::new();

        while !parser.consume_a(Token::RBrace) {
            fields.push(match parser.peek(0)? {
                // { name = Exp }
                Token::Name(_) if parser.peek(1) == Ok(Token::Op(Op::Eq)) => {
                    let key = parser.node(|p| p.parse_name().map(Exp::String))?;

                    parser.consume()?;

                    let field = Field::new(Some(key), parser.parse_exp()?);

                    parser.consume_a(Token::Semicolon);
                    parser.consume_a(Token::Comma);

                    field
                }

                // { [Exp] = Exp }
                Token::LBracket => {
                    parser.consume()?;

                    let key = parser.parse_exp()?;

                    parser.expect(Token::RBracket)?;

                    parser.expect(Op::Eq)?;

                    let value = parser.parse_exp()?;

                    parser.consume_a(Token::Semicolon);
                    parser.consume_a(Token::Comma);

                    Field::new(Some(key), value)
                }

                // { Exp }
                _ => {
                    let value = parser.parse_exp()?;

                    parser.consume_a(Token::Semicolon);
                    parser.consume_a(Token::Comma);

                    Field::new(None, value)
                }
            })
        }

        Ok(TableConstructor::new(fields).into())
    }
}

pub struct UnaryParselet;

impl Nud for UnaryParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> Result<Exp, String> {
        let op = match token {
            Token::Op(Op::Len) => UnOp::Len,
            Token::Op(Op::Not) => UnOp::Not,
            Token::Op(Op::Sub) => UnOp::Neg,

            _ => unreachable!(),
        };

        Ok(Unary::new(op, parser.parse_exp_prec(Precedence::Unary)?).into())
    }
}
