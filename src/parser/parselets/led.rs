use crate::ast::Exp;
use crate::ast::exps::{Binary, FunctionCall, Index, Member, MethodCall};
use crate::ast::exps::binary::BinOp;
use crate::ast::node::Node;
use crate::lexer::{Op, Token};
use crate::parser::{Parser, Precedence};
use crate::parser::parselets::Led;

pub struct AccessParselet;

impl Led for AccessParselet {
    fn parse(&self, parser: &mut Parser, lhs: Node<Exp>, token: Token) -> Result<Exp, String> {
        match token {
            // foo[Exp]
            Token::LBracket => {
                let tracker = parser.start_node()?;

                let exp = parser.parse_exp()?;

                parser.expect(Token::RBracket)?;

                Ok(parser.produce_node(tracker, Index::new(lhs, exp)).into())
            }

            // foo.Name
            Token::Op(Op::Dot) => {
                let tracker = parser.start_node()?;

                let name = parser.parse_name()?;
                Ok(parser.produce_node(tracker, Member::new(lhs, name)).into())
            }

            _ => unreachable!()
        }
    }

    fn get_precedence(&self) -> Precedence {
        Precedence::None
    }
}

pub struct AdditiveParselet;

impl Led for AdditiveParselet {
    fn parse(&self, parser: &mut Parser, lhs: Node<Exp>, token: Token) -> Result<Exp, String> {
        let tracker = parser.start_node()?;

        let op = match token {
            Token::Op(Op::Add) => BinOp::Add,
            Token::Op(Op::Sub) => BinOp::Sub,
            _ => unreachable!()
        };

        let rhs = parser.parse_exp_prec(self.get_precedence())?;

        Ok(parser.produce_node(tracker, Binary::new(lhs, op, rhs)).into())
    }

    fn get_precedence(&self) -> Precedence {
        Precedence::Additive
    }
}

pub struct AndParselet;

impl Led for AndParselet {
    fn parse(&self, parser: &mut Parser, lhs: Node<Exp>, token: Token) -> Result<Exp, String> {
        assert_eq!(Token::Op(Op::And), token);

        let tracker = parser.start_node()?;

        let rhs = parser.parse_exp_prec(self.get_precedence())?;

        Ok(parser.produce_node(tracker, Binary::new(lhs, BinOp::And, rhs)).into())
    }

    fn get_precedence(&self) -> Precedence {
        Precedence::And
    }
}

pub struct ConcatParselet;

impl Led for ConcatParselet {
    fn parse(&self, parser: &mut Parser, lhs: Node<Exp>, token: Token) -> Result<Exp, String> {
        assert_eq!(Token::Op(Op::DotDot), token);

        let tracker = parser.start_node()?;

        // Right associative so pass one lower precedence level than us
        let rhs = parser.parse_exp_prec(Precedence::Comparative)?;

        Ok(parser.produce_node(tracker, Binary::new(lhs, BinOp::Concat, rhs)).into())
    }

    fn get_precedence(&self) -> Precedence {
        Precedence::Concat
    }
}

pub struct ComparativeParselet;

impl Led for ComparativeParselet {
    fn parse(&self, parser: &mut Parser, lhs: Node<Exp>, token: Token) -> Result<Exp, String> {
        let tracker = parser.start_node()?;

        let op = match token {
            Token::Op(Op::EqEq) => BinOp::Eq,
            Token::Op(Op::Gt) => BinOp::Gt,
            Token::Op(Op::GtEq) => BinOp::GtEq,
            Token::Op(Op::Lt) => BinOp::Lt,
            Token::Op(Op::LtEq) => BinOp::LtEq,
            Token::Op(Op::Ne) => BinOp::Ne,
            _ => unreachable!()
        };

        let rhs = parser.parse_exp_prec(self.get_precedence())?;

        Ok(parser.produce_node(tracker, Binary::new(lhs, op, rhs)).into())
    }

    fn get_precedence(&self) -> Precedence {
        Precedence::Comparative
    }
}

pub struct ExponentiationParselet;

impl Led for ExponentiationParselet {
    fn parse(&self, parser: &mut Parser, lhs: Node<Exp>, token: Token) -> Result<Exp, String> {
        assert_eq!(Token::Op(Op::Exp), token);

        let tracker = parser.start_node()?;

        // Right associative so pass one lower precedence level than us
        let rhs = parser.parse_exp_prec(Precedence::Unary)?;

        Ok(parser.produce_node(tracker, Binary::new(lhs, BinOp::Exp, rhs)).into())
    }

    fn get_precedence(&self) -> Precedence {
        Precedence::Exponentiation
    }
}

pub struct FunctionCallParselet;

impl Led for FunctionCallParselet {
    fn parse(&self, parser: &mut Parser, lhs: Node<Exp>, token: Token) -> Result<Exp, String> {
        let tracker = parser.start_node()?;

        let args = parser.parse_args(token)?;

        Ok(parser.produce_node(tracker, FunctionCall::new(lhs, args)).into())
    }

    fn get_precedence(&self) -> Precedence {
        Precedence::None
    }
}

pub struct MethodCallParselet;

impl Led for MethodCallParselet {
    fn parse(&self, parser: &mut Parser, lhs: Node<Exp>, token: Token) -> Result<Exp, String> {
        assert_eq!(Token::Op(Op::Colon), token);

        let tracker = parser.start_node()?;

        let name = parser.parse_name()?;

        let args = {
            let (token, _) = parser.consume()?;

            parser.parse_args(token)?
        };

        Ok(parser.produce_node(tracker, MethodCall::new(lhs, name, args)).into())
    }

    fn get_precedence(&self) -> Precedence {
        Precedence::None
    }
}

pub struct MultiplicativeParselet;

impl Led for MultiplicativeParselet {
    fn parse(&self, parser: &mut Parser, lhs: Node<Exp>, token: Token) -> Result<Exp, String> {
        let tracker = parser.start_node()?;

        let op = match token {
            Token::Op(Op::Mod) => BinOp::Mod,
            Token::Op(Op::Mul) => BinOp::Mul,
            Token::Op(Op::Div) => BinOp::Div,
            _ => unreachable!()
        };

        let rhs = parser.parse_exp_prec(self.get_precedence())?;

        Ok(parser.produce_node(tracker, Binary::new(lhs, op, rhs)).into())
    }

    fn get_precedence(&self) -> Precedence {
        Precedence::Multiplicative
    }
}

pub struct OrParselet;

impl Led for OrParselet {
    fn parse(&self, parser: &mut Parser, lhs: Node<Exp>, token: Token) -> Result<Exp, String> {
        assert_eq!(Token::Op(Op::Or), token);

        let tracker = parser.start_node()?;

        let rhs = parser.parse_exp_prec(self.get_precedence())?;

        Ok(parser.produce_node(tracker, Binary::new(lhs, BinOp::Or, rhs)).into())
    }

    fn get_precedence(&self) -> Precedence {
        Precedence::Or
    }
}