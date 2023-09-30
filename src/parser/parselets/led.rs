use crate::{
    ast::{
        exps::{binary::BinOp, Binary, FunctionCall, Index, Member, MethodCall},
        node::Node,
        Exp,
    },
    lexer::{Op, Token},
    parser::{parselets::Led, Parser, Precedence, Result},
};

pub struct AccessParselet;

impl Led for AccessParselet {
    fn parse<'a>(
        &self,
        parser: &mut Parser<'a>,
        lhs: Node<&'a Exp>,
        token: Token<'a>,
    ) -> Result<'a, Exp<'a>> {
        match token {
            // foo[Exp]
            Token::LBracket => {
                let exp = parser.node(Parser::parse_exp)?;

                parser.expect(Token::RBracket)?;

                Ok(Index::new(lhs, exp).into())
            }

            // foo.Name
            Token::Op(Op::Dot) => {
                let name = parser.parse_name()?;

                Ok(Member::new(lhs, name).into())
            }

            _ => unreachable!(),
        }
    }

    fn get_precedence(&self) -> Precedence {
        Precedence::None
    }
}

pub struct AdditiveParselet;

impl Led for AdditiveParselet {
    fn parse<'a>(
        &self,
        parser: &mut Parser<'a>,
        lhs: Node<&'a Exp>,
        token: Token<'a>,
    ) -> Result<'a, Exp<'a>> {
        let op = match token {
            Token::Op(Op::Add) => BinOp::Add,
            Token::Op(Op::Sub) => BinOp::Sub,
            _ => unreachable!(),
        };

        let rhs = parser.node(|p| p.parse_exp_prec(self.get_precedence()))?;

        Ok(Binary::new(lhs, op, rhs).into())
    }

    fn get_precedence(&self) -> Precedence {
        Precedence::Additive
    }
}

pub struct AndParselet;

impl Led for AndParselet {
    fn parse<'a>(
        &self,
        parser: &mut Parser<'a>,
        lhs: Node<&'a Exp>,
        token: Token<'a>,
    ) -> Result<'a, Exp<'a>> {
        assert_eq!(Token::Op(Op::And), token);

        let rhs = parser.node(|p| p.parse_exp_prec(self.get_precedence()))?;

        Ok(Binary::new(lhs, BinOp::And, rhs).into())
    }

    fn get_precedence(&self) -> Precedence {
        Precedence::And
    }
}

pub struct ConcatParselet;

impl Led for ConcatParselet {
    fn parse<'a>(
        &self,
        parser: &mut Parser<'a>,
        lhs: Node<&'a Exp>,
        token: Token<'a>,
    ) -> Result<'a, Exp<'a>> {
        assert_eq!(Token::Op(Op::DotDot), token);

        // Right associative so pass one lower precedence level than us
        let rhs = parser.node(|p| p.parse_exp_prec(Precedence::Comparative))?;

        Ok(Binary::new(lhs, BinOp::Concat, rhs).into())
    }

    fn get_precedence(&self) -> Precedence {
        Precedence::Concat
    }
}

pub struct ComparativeParselet;

impl Led for ComparativeParselet {
    fn parse<'a>(
        &self,
        parser: &mut Parser<'a>,
        lhs: Node<&'a Exp>,
        token: Token<'a>,
    ) -> Result<'a, Exp<'a>> {
        let op = match token {
            Token::Op(Op::EqEq) => BinOp::Eq,
            Token::Op(Op::Gt) => BinOp::Gt,
            Token::Op(Op::GtEq) => BinOp::GtEq,
            Token::Op(Op::Lt) => BinOp::Lt,
            Token::Op(Op::LtEq) => BinOp::LtEq,
            Token::Op(Op::Ne) => BinOp::Ne,
            _ => unreachable!(),
        };

        let rhs = parser.node(|p| p.parse_exp_prec(self.get_precedence()))?;

        Ok(Binary::new(lhs, op, rhs).into())
    }

    fn get_precedence(&self) -> Precedence {
        Precedence::Comparative
    }
}

pub struct ExponentiationParselet;

impl Led for ExponentiationParselet {
    fn parse<'a>(
        &self,
        parser: &mut Parser<'a>,
        lhs: Node<&'a Exp>,
        token: Token<'a>,
    ) -> Result<'a, Exp<'a>> {
        assert_eq!(Token::Op(Op::Exp), token);

        // Right associative so pass one lower precedence level than us
        let rhs = parser.node(|p| p.parse_exp_prec(Precedence::Unary))?;

        Ok(Binary::new(lhs, BinOp::Exp, rhs).into())
    }

    fn get_precedence(&self) -> Precedence {
        Precedence::Exponentiation
    }
}

pub struct FunctionCallParselet;

impl Led for FunctionCallParselet {
    fn parse<'a>(
        &self,
        parser: &mut Parser<'a>,
        lhs: Node<&'a Exp>,
        token: Token<'a>,
    ) -> Result<'a, Exp<'a>> {
        let args = parser.parse_args(token)?;

        Ok(FunctionCall::new(lhs, args.into_bump_slice()).into())
    }

    fn get_precedence(&self) -> Precedence {
        Precedence::None
    }
}

pub struct MethodCallParselet;

impl Led for MethodCallParselet {
    fn parse<'a>(
        &self,
        parser: &mut Parser<'a>,
        lhs: Node<&'a Exp>,
        token: Token<'a>,
    ) -> Result<'a, Exp<'a>> {
        assert_eq!(Token::Op(Op::Colon), token);

        let name = parser.parse_name()?;

        let args = {
            let token = parser.consume()?;

            parser.parse_args(*token)?
        };

        Ok(MethodCall::new(lhs, name, args.into_bump_slice()).into())
    }

    fn get_precedence(&self) -> Precedence {
        Precedence::None
    }
}

pub struct MultiplicativeParselet;

impl Led for MultiplicativeParselet {
    fn parse<'a>(
        &self,
        parser: &mut Parser<'a>,
        lhs: Node<&'a Exp>,
        token: Token<'a>,
    ) -> Result<'a, Exp<'a>> {
        let op = match token {
            Token::Op(Op::Mod) => BinOp::Mod,
            Token::Op(Op::Mul) => BinOp::Mul,
            Token::Op(Op::Div) => BinOp::Div,
            _ => unreachable!(),
        };

        let rhs = parser.node(|p| p.parse_exp_prec(self.get_precedence()))?;

        Ok(Binary::new(lhs, op, rhs).into())
    }

    fn get_precedence(&self) -> Precedence {
        Precedence::Multiplicative
    }
}

pub struct OrParselet;

impl Led for OrParselet {
    fn parse<'a>(
        &self,
        parser: &mut Parser<'a>,
        lhs: Node<&'a Exp>,
        token: Token<'a>,
    ) -> Result<'a, Exp<'a>> {
        assert_eq!(Token::Op(Op::Or), token);

        let rhs = parser.node(|p| p.parse_exp_prec(self.get_precedence()))?;

        Ok(Binary::new(lhs, BinOp::Or, rhs).into())
    }

    fn get_precedence(&self) -> Precedence {
        Precedence::Or
    }
}
