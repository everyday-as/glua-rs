use crate::ast::Exp;
use crate::ast::exps::{TableConstructor, Unary};
use crate::ast::exps::table::Field;
use crate::ast::exps::unary::UnOp;
use crate::ast::node::Node;
use crate::lexer::{Keyword, Literal, Op, Token};
use crate::parser::parselets::Nud;
use crate::parser::{NodeTracker, Parser, Precedence};

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
        let end = parser.rewind_stack.last().unwrap().1.end;

        match token {
            Token::Literal(literal) => match literal {
                Literal::Bool(value) => {
                    let start = end - value.to_string().len();
                    Ok(Exp::Bool(parser.produce_node_with_span(start..end, value)))
                },
                Literal::Nil => Ok(Exp::Nil),
                Literal::Number(value) => {
                    let start = end - value.to_string().len();
                    Ok(Exp::Number(parser.produce_node_with_span(start..end, value)))
                },
                Literal::String(value) => {
                    let start = end - value.len();
                    Ok(Exp::String(parser.produce_node_with_span(start..end, value)))
                }
            },

            _ => unreachable!()
        }
    }
}

pub struct NameParselet;

impl Nud for NameParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> Result<Exp, String> {
        if let Token::Name(name) = token {
            let end = parser.rewind_stack.last().unwrap().1.end;
            let start = end - name.len();

            return Ok(Exp::Ref(parser.produce_node_with_span(start..end, name)));
        }

        unreachable!();
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
                Token::Name(name) if parser.peek(1) == Ok(Token::Op(Op::Eq)) => {
                    let tracker = parser.start_node()?;

                    parser.consume()?;
                    parser.consume()?;

                    let field = Field::new(Some(parser.produce_node(tracker, Exp::String(parser.produce_node(tracker, name)))), parser.parse_exp()?);

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

                    parser.consume_a(Token::Comma);

                    Field::new(Some(key), value)
                }

                // { Exp }
                _ => {
                    let value = parser.parse_exp()?;

                    parser.consume_a(Token::Comma);

                    Field::new(None, value)
                }
            })
        }

        let tracker = parser.start_node()?;

        Ok(parser.produce_node(tracker, TableConstructor::new(fields)).into())
    }
}

pub struct UnaryParselet;

impl Nud for UnaryParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> Result<Exp, String> {
        let tracker = parser.start_node()?;

        let op = match token {
            Token::Op(Op::Len) => UnOp::Len,
            Token::Op(Op::Not) => UnOp::Not,
            Token::Op(Op::Sub) => UnOp::Neg,

            _ => unreachable!()
        };

        let unary = Unary::new(op, parser.parse_exp_prec(Precedence::Unary)?);
        Ok(parser.produce_node(tracker, unary).into())
    }
}
