pub use logos::Span;
use logos::{Lexer, Logos};
use std::collections::VecDeque;
use std::ops::Range;

use crate::ast::exps::*;
use crate::ast::node::Node;
use crate::ast::stats::*;
use crate::ast::*;
use crate::lexer::*;
use crate::parser::parselets::nud::TableConstructorParselet;
use crate::parser::parselets::{led, nud, Led, Nud};

mod parselets;

pub type SpannedToken<'a> = (Token<'a>, Span);

pub struct Parser<'source> {
    tokens: VecDeque<SpannedToken<'source>>,
    rewind_stack: Vec<SpannedToken<'source>>,
    last_span: Range<usize>,
}

impl<'source> Parser<'source> {
    pub fn new(tokens: Vec<SpannedToken<'source>>) -> Self {
        Self {
            tokens: tokens
                .into_iter()
                .filter_map(|(t, span)| match t {
                    Token::Comment(_) => None,

                    token => Some((token, span)),
                })
                .collect(),
            rewind_stack: Vec::new(),
            last_span: 0..0,
        }
    }

    pub fn parse_chunk(&mut self) -> Result<Block, String> {
        let block = self.parse_block()?;

        match self.tokens.pop_front() {
            None => Ok(block),
            Some(token) => Err(format!("Unexpected {:?}, expected EOF", token)),
        }
    }

    fn parse_block(&mut self) -> Result<Block, String> {
        let mut stats = Vec::new();

        // Rewind here, because Lua has SYNTACTICALLY ASCENDED THE MORTAL FUCKING PLANE
        while let Some(stat) = self.with_rewind(
            |p| p.parse_stat(),
            |e| e == "Unexpected EOF" || e.ends_with("expected stat")
        )? {
            self.consume_a(Token::Semicolon);

            stats.push(stat);
        }

        if self.next_is_in(&[Keyword::Break, Keyword::Continue, Keyword::Return]) {
            stats.push(self.node(|p| p.parse_last_stat())?);

            self.consume_a(Token::Semicolon);
        }

        Ok(stats)
    }

    pub fn parse_stat(&mut self) -> Result<Node<Stat>, String> {
        self.node(Self::parse_stat_impl)
    }

    fn parse_stat_impl(&mut self) -> Result<Stat, String> {
        let stat = match self.peek(0)? {
            Token::Name(_) | Token::LParens => {
                // Ambiguously an `Assignment` or a `FunctionCall`, so we have to rewind
                match self
                    .with_rewind(
                        |parser| {
                            let exp = parser.node(|p| p.parse_var())
                                .map_err(|_| String::new())?;

                            // Eq or Comma denotes an assignment expression
                            if parser.next_is_in(&[Token::Comma, Token::Op(Op::Eq)]) {
                                Ok(exp)
                            } else {
                                // Otherwise we rewind, signalled by an empty string
                                // TODO: improve this
                                Err(String::new())
                            }
                        },
                        |e| e.is_empty()
                    )?
                    .clone()
                {
                    // `Assignment`
                    Some(var) => {
                        let mut vars = vec![var];

                        while self.consume_a(Token::Comma) {
                            vars.push(self.node(Self::parse_var)?)
                        }

                        self.expect(Op::Eq)?;

                        let exps = self.parse_list(Self::parse_exp)?;

                        Ok(Assignment::new(vars, exps).into())
                    }

                    // `FunctionCall`
                    None => {
                        let token = self.peek(0)?;

                        match self.parse_prefix_exp()? {
                            Exp::FunctionCall(call) => Ok(Stat::FunctionCall(call)),

                            Exp::MethodCall(call) => Ok(Stat::MethodCall(call)),

                            _ => Err(format!("Unexpected `{:?}`, expected functioncall", token)),
                        }
                    }
                }
            }

            Token::Keyword(keyword) => {
                self.consume()?;

                match keyword {
                    // do block end
                    Keyword::Do => {
                        let body = self.parse_block()?;

                        self.expect(Keyword::End)?;

                        Ok(Do::new(body).into())
                    }

                    // while exp do block end
                    Keyword::While => {
                        let cond = self.parse_exp()?;

                        self.expect(Keyword::Do)?;

                        let body = self.parse_block()?;

                        self.expect(Keyword::End)?;

                        Ok(While::new(cond, body).into())
                    }

                    // repeat block until exp
                    Keyword::Repeat => {
                        let body = self.parse_block()?;

                        self.expect(Keyword::Until)?;

                        let cond = self.parse_exp()?;

                        Ok(RepeatUntil::new(body, cond).into())
                    }

                    // if exp then block {elseif exp then block} [else block] end
                    Keyword::If => {
                        let cond = self.parse_exp()?;

                        self.expect(Keyword::Then)?;

                        let body = self.parse_block()?;

                        let mut else_ifs = Vec::new();

                        while self.consume_a(Keyword::ElseIf) {
                            let cond = self.parse_exp()?;

                            self.expect(Keyword::Then)?;

                            let body = self.parse_block()?;

                            else_ifs.push((cond, body));
                        }

                        let else_block = match self.consume_a(Keyword::Else) {
                            true => Some(self.parse_block()?),
                            false => None,
                        };

                        self.expect(Keyword::End)?;

                        Ok(IfElse::new(cond, body, else_ifs, else_block).into())
                    }

                    Keyword::For => match self.peek(1)? {
                        // for Name `=´ exp `,´ exp [`,´ exp] do block end
                        Token::Op(Op::Eq) => {
                            let init = {
                                let name = self.parse_name()?;

                                self.expect(Op::Eq)?;

                                let exp = self.parse_exp()?;

                                (name, exp)
                            };

                            self.expect(Token::Comma)?;

                            let test = self.parse_exp()?;

                            let update = match self.consume_a(Token::Comma) {
                                true => Some(self.parse_exp()?),
                                false => None,
                            };

                            self.expect(Keyword::Do)?;

                            let body = self.parse_block()?;

                            self.expect(Keyword::End)?;

                            Ok(For::new(init, test, update, body).into())
                        }

                        // for namelist in explist do block end
                        _ => {
                            let names = self.parse_list(Self::parse_name)?;

                            self.expect(Keyword::In)?;

                            let exps = self.parse_list(Self::parse_exp)?;

                            self.expect(Keyword::Do)?;

                            let body = self.parse_block()?;

                            self.expect(Keyword::End)?;

                            Ok(ForIn::new(names, exps, body).into())
                        }
                    },

                    // function funcname funcbody
                    Keyword::Function => {
                        let name = {
                            let parts = self
                                .parse_delimited(Op::Dot, Self::parse_name, |token| {
                                    [Token::LParens, Token::Op(Op::Colon)].contains(&token)
                                })
                                .unwrap();

                            let mut name = parts.join(".");

                            if self.consume_a(Op::Colon) {
                                name.push(':');
                                name.push_str(&self.parse_name()?)
                            }

                            name
                        };

                        let body = self.node(|p| p.parse_function())?;

                        Ok(FunctionDef::new(false, name, body).into())
                    }

                    Keyword::Local => match self.peek(0)? {
                        // local function Name funcbody
                        Token::Keyword(Keyword::Function) => {
                            self.consume()?;

                            let name = self.parse_name()?;

                            let body = self.node(|p| p.parse_function())?;

                            Ok(FunctionDef::new(true, name, body).into())
                        }

                        // local namelist [`=´ explist]
                        _ => {
                            let names = self.parse_list(Self::parse_name)?;

                            let init_exps = match self.consume_a(Op::Eq) {
                                true => Some(self.parse_list(Self::parse_exp)?),
                                false => None,
                            };

                            Ok(VarDef::new(names, init_exps).into())
                        }
                    },

                    Keyword::Goto => match self.peek(0)? {
                        // Goto must be followed by a Token::Name
                        Token::Name(label) => {
                            // goto Name
                            self.consume()?;

                            Ok(Goto::new(label.to_owned()).into())
                        }
                        _ => Err(format!("Unexpected `{:?}`, expected name", keyword)),
                    },

                    _ => Err(format!("Unexpected `{:?}`, expected stat", keyword)),
                }
            }

            Token::Label(name) => {
                self.consume()?;

                Ok(Label::new(name.to_owned()).into())
            }

            token => Err(format!("Unexpected `{:?}`, expected stat", token)),
        };

        stat
    }

    fn parse_last_stat(&mut self) -> Result<Stat, String> {
        let (token, _) = self.consume()?;

        let stat = match token {
            Token::Keyword(Keyword::Return) => {
                match self.with_rewind(
                    |parser| parser.parse_list(|p| p.parse_exp()),
                    |e| e.ends_with("expected expression")
                )? {
                    Some(exps) => Ok(Return::new(exps).into()),
                    None => Ok(Return::new(Vec::new()).into()),
                }
            }

            Token::Keyword(Keyword::Break) => Ok(Stat::Break),

            // GMod specific
            Token::Keyword(Keyword::Continue) => Ok(Stat::Continue),

            token => Err(format!(
                "Unexpected `{:?}`, expected return, continue or break",
                token
            )),
        };

        stat
    }

    pub fn parse_exp(&mut self) -> Result<Node<Exp>, String> {
        self.parse_exp_prec(Precedence::None)
    }

    fn parse_exp_prec(&mut self, min_precedence: Precedence) -> Result<Node<Exp>, String> {
        let mut lhs = self.node(|p| match get_nud_parselet(&p.peek(0)?) {
            Some(parselet) => {
                let (token, _) = p.consume()?;

                parselet.parse(p, token)
            }

            None => p.parse_prefix_exp(),
        })?;

        while min_precedence < self.get_precedence() {
            let (token, _) = self.consume()?;

            lhs = match get_led_parselet(&token) {
                Some(parselet) => self.node(|p| parselet.parse(p, lhs, token))?,

                None => return Err(format!("Unexpected `{:?}` in expression", token)),
            }
        }

        Ok(lhs)
    }

    fn parse_prefix_exp(&mut self) -> Result<Exp, String> {
        let mut lhs = self.node(|p| {
            let (token, _) = p.consume()?;

            match get_prefix_nud_parselet(&token) {
                Some(parselet) => parselet.parse(p, token),

                None => Err(format!("Unexpected `{:?}`, expected expression", token)),
            }
        })?;

        while let Ok(next) = self.peek(0) {
            if let Some(parselet) = get_prefix_led_parselet(&next) {
                lhs = self.node(|p| {
                    let (token, _) = p.consume()?;

                    parselet.parse(p, lhs, token)
                })?;
            } else {
                break;
            }
        }

        Ok(lhs.inner)
    }

    /// Parses a var, basically a more selective prefixexp
    fn parse_var(&mut self) -> Result<Exp, String> {
        let exp = self.parse_prefix_exp()?;

        match exp {
            Exp::Index(_) => Ok(exp),

            Exp::Member(_) => Ok(exp),

            Exp::Ref(_) => Ok(exp),

            _ => Err("Unexpected prefixexp, expecting var".to_owned()),
        }
    }

    fn parse_function(&mut self) -> Result<Function, String> {
        self.expect(Token::LParens)?;

        let mut params = self.parse_delimited(Token::Comma, Parser::parse_name, |token| {
            Token::Ellipsis == token || Token::RParens == token
        })?;

        if self.consume_a(Token::Ellipsis) {
            params.push("...".to_owned());
        }

        self.expect(Token::RParens)?;

        let body = self.parse_block()?;

        self.expect(Keyword::End)?;

        Ok(Function::new(params, body))
    }

    // <Helpers>
    fn get_precedence(&self) -> Precedence {
        match self.peek(0) {
            Ok(token) => match get_led_parselet(&token) {
                Some(parselet) => parselet.get_precedence(),

                None => match get_prefix_led_parselet(&token) {
                    Some(parselet) => parselet.get_precedence(),

                    None => Precedence::None,
                },
            },

            Err(_) => Precedence::None,
        }
    }

    fn peek(&self, n: usize) -> Result<Token<'source>, String> {
        match self.tokens.get(n) {
            Some((token, _)) => Ok(token.clone()),

            None => Err("Unexpected EOF".to_owned()),
        }
    }

    fn consume(&mut self) -> Result<SpannedToken<'source>, String> {
        self.tokens
            .pop_front()
            .ok_or("Unexpected EOF".to_owned())
            .and_then(|token| {
                self.rewind_stack.push(token.clone());
                self.last_span = token.1.clone();

                Ok(token)
            })
    }

    fn expect<'a, E>(&mut self, expected: E) -> Result<(), String>
        where
            E: Into<Token<'a>>,
    {
        let (expected, (got, _)) = (expected.into(), self.consume()?);

        if got == expected {
            Ok(())
        } else {
            Err(format!("Unexpected `{:?}`, expected `{:?}`", got, expected))
        }
    }

    fn consume_a<'a, E>(&mut self, expected: E) -> bool
        where
            E: Into<Token<'a>>,
    {
        return if self.next_is(expected) {
            // If `next_is` returns `true` but `consume` fails something very, very bad has happened
            self.consume().expect("Internal error");

            true
        } else {
            false
        };
    }

    fn next_is<'a, E>(&mut self, expected: E) -> bool
        where
            E: Into<Token<'a>>,
    {
        match self.peek(0) {
            Ok(got) => got == expected.into(),

            Err(_) => false,
        }
    }

    fn next_is_in<'a, P>(&mut self, possibilities: &[P]) -> bool
        where
            P: Into<Token<'a>> + Clone,
    {
        for possibility in possibilities {
            if self.next_is(possibility.clone().into()) {
                return true;
            }
        }

        false
    }

    fn with_rewind<T, F, C>(
        &mut self,
        func: F,
        can_rewind: C
    ) -> Result<Option<T>, String>
        where
            F: FnOnce(&mut Parser) -> Result<T, String>,
            C: FnOnce(&str) -> bool,
    {
        let rewind_to = self.rewind_stack.len();

        match func(self) {
            Ok(result) => Ok(Some(result)),
            Err(err) => match can_rewind(&err) {
                true => {
                    while self.rewind_stack.len() > rewind_to {
                        self.tokens.push_front(self.rewind_stack.pop().unwrap());
                    }

                    Ok(None)
                }

                false => Err(err),
            }
        }
    }

    fn node<T>(
        &mut self,
        f: impl FnOnce(&mut Self) -> Result<T, String>,
    ) -> Result<Node<T>, String> {
        let start = self
            .tokens
            .get(0)
            .ok_or_else(|| "Unexpected EOF".to_owned())?
            .1
            .start;

        let inner = f(self)?;

        let span = start..self.last_span.end;

        Ok(Node { span, inner })
    }
    // </Helpers>

    // <Parse Helpers>
    /// Parse function / method arguments
    fn parse_args(&mut self, token: Token) -> Result<Vec<Node<Exp>>, String> {
        match token {
            // function(arg, arg2)
            Token::LParens => match self.consume_a(Token::RParens) {
                true => Ok(Vec::new()),
                false => {
                    let args = self.parse_list(Self::parse_exp)?;

                    self.expect(Token::RParens)?;

                    Ok(args)
                }
            },

            // function{ table }
            Token::LBrace => {
                let start = self.last_span.start;

                let inner = TableConstructorParselet.parse(self, token)?;

                let span = start..self.last_span.end;

                Ok(vec![Node {
                    span,
                    inner,
                }])
            }

            // function"string"
            Token::Literal(Literal::String(arg)) => {
                Ok(vec![Node {
                    span: self.last_span.clone(),
                    inner: Exp::String(arg.into_owned()),
                }])
            }

            token => Err(format!("Unexpected {:?}, expected args", token)),
        }
    }

    /// Parse a name
    fn parse_name(&mut self) -> Result<String, String> {
        let (token, _) = self.consume()?;

        match token {
            Token::Name(name) => Ok(name.to_owned()),
            Token::Keyword(Keyword::Goto) => Ok(String::from("goto")),

            token => Err(format!("Unexpected `{:?}`, expected name", token)),
        }
    }

    fn parse_delimited<T, D, P, IE>(
        &mut self,
        delim: D,
        parse: P,
        is_end: IE,
    ) -> Result<Vec<T>, String>
        where
            T: 'source,
            D: Into<Token<'source>>,
            P: Fn(&mut Parser<'source>) -> Result<T, String>,
            IE: Fn(Token) -> bool,
    {
        let (mut items, delim) = (Vec::new(), delim.into());

        while !is_end(self.peek(0)?) {
            items.push(parse(self)?);

            if !is_end(self.peek(0)?) {
                self.expect(delim.clone())?;
            }
        }

        Ok(items)
    }

    fn parse_list<T, P>(&mut self, parse: P) -> Result<Vec<T>, String>
        where
            P: Fn(&mut Parser<'source>) -> Result<T, String>,
    {
        let mut items = Vec::new();

        loop {
            items.push(parse(self)?);

            if !self.consume_a(Token::Comma) {
                break;
            }
        }

        Ok(items)
    }
    // </Parse Helpers>
}

fn get_nud_parselet(token: &Token) -> Option<&'static dyn Nud> {
    match token {
        Token::Ellipsis => Some(&nud::EllipsisParselet),

        Token::Keyword(Keyword::Function) => Some(&nud::FunctionParselet),

        Token::LBrace => Some(&nud::TableConstructorParselet),

        Token::Literal(_) => Some(&nud::LiteralParselet),

        Token::Op(Op::Len) | Token::Op(Op::Not) | Token::Op(Op::Sub) => Some(&nud::UnaryParselet),

        _ => None,
    }
}

fn get_led_parselet(token: &Token) -> Option<&'static dyn Led> {
    match token {
        Token::Op(Op::Exp) => Some(&led::ExponentiationParselet),

        Token::Op(Op::Mod) | Token::Op(Op::Mul) | Token::Op(Op::Div) => {
            Some(&led::MultiplicativeParselet)
        }

        Token::Op(Op::Add) | Token::Op(Op::Sub) => Some(&led::AdditiveParselet),

        Token::Op(Op::DotDot) => Some(&led::ConcatParselet),

        Token::Op(Op::Lt)
        | Token::Op(Op::Gt)
        | Token::Op(Op::LtEq)
        | Token::Op(Op::GtEq)
        | Token::Op(Op::Ne)
        | Token::Op(Op::EqEq) => Some(&led::ComparativeParselet),

        Token::Op(Op::And) => Some(&led::AndParselet),

        Token::Op(Op::Or) => Some(&led::OrParselet),

        _ => None,
    }
}

fn get_prefix_nud_parselet(token: &Token) -> Option<&'static dyn Nud> {
    match token {
        Token::LParens => Some(&nud::ParensParselet),

        Token::Keyword(Keyword::Goto) | Token::Name(_) => Some(&nud::NameParselet),

        _ => None,
    }
}

fn get_prefix_led_parselet(token: &Token) -> Option<&'static dyn Led> {
    match token {
        Token::LBracket | Token::Op(Op::Dot) => Some(&led::AccessParselet),

        Token::LParens | Token::LBrace | Token::Literal(Literal::String(_)) => {
            Some(&led::FunctionCallParselet)
        }

        Token::Op(Op::Colon) => Some(&led::MethodCallParselet),

        _ => None,
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum Precedence {
    None,
    Or,
    And,
    Comparative,
    Concat,
    Additive,
    Multiplicative,
    Unary,
    Exponentiation,
}
