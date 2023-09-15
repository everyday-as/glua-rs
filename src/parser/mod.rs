pub use logos::Span;
use std::collections::VecDeque;

use crate::ast::exps::*;
use crate::ast::node::Node;
use crate::ast::stats::*;
use crate::ast::*;
use crate::lexer::*;
use crate::parser::parselets::nud::TableConstructorParselet;
use crate::parser::parselets::{led, nud, Led, Nud};

mod parselets;

pub type SpannedToken = (Token, Span);

pub struct Parser {
    tokens: VecDeque<SpannedToken>,
    rewind_stack: Vec<SpannedToken>,
    spans: Vec<usize>,
}

impl Parser {
    pub fn new(tokens: Vec<SpannedToken>) -> Self {
        // let mut cur_line = 1;

        Self {
            tokens: tokens
                .into_iter()
                .filter_map(|(t, span)| match t {
                    Token::Whitespace(_) => {
                        None
                    }

                    Token::Comment(_) => {
                        None
                    },

                    token => Some((token, span)),
                })
                .collect(),
            rewind_stack: Vec::new(),
            spans: Vec::new(),
        }
    }

    pub fn parse_chunk(&mut self) -> Result<Block, String> {
        let block = self.parse_block()?;

        match self.tokens.is_empty() {
            true => Ok(block),
            false => Err(format!(
                "Unexpected {:?}, expected EOF",
                self.tokens.pop_front().unwrap()
            )),
        }
    }

    fn start_node(&mut self) -> Result<(), String> {
        match self.tokens.front() {
            Some(token) => {
                self.spans.push(token.1.start);

                Ok(())
            }
            None => Err("Unexpected EOF".to_owned()),
        }
    }

    fn fork_node(&mut self) -> Result<(), String> {
        self.spans.last().copied().map(|span| self.spans.push(span));

        Ok(())
    }

    fn consume_node(&mut self) -> Result<(), String> {
        self.spans.pop();

        Ok(())
    }

    fn produce_node<T>(&mut self, inner: T) -> Node<T> {
        let start = self.spans.pop().unwrap();
        let end = self.rewind_stack.last().unwrap().1.end;

        Node {
            span: start..end,
            inner,
        }
    }

    fn produce_node_with_span<T>(&self, span: Span, inner: T) -> Node<T> {
        Node { span, inner }
    }

    fn parse_block(&mut self) -> Result<Block, String> {
        let mut stats = Vec::new();

        // Rewind here, because Lua has SYNTACTICALLY ASCENDED THE MORTAL FUCKING PLANE
        while let Some(stat) = self.with_rewind(Self::parse_stat, |e| {
            e == "Unexpected EOF" || e.ends_with("expected stat")
        })? {
            self.consume_a(Token::Semicolon);

            stats.push(stat);
        }

        if self.next_is_in(&[Keyword::Break, Keyword::Continue, Keyword::Return]) {
            stats.push(self.parse_last_stat()?);

            self.consume_a(Token::Semicolon);
        }

        Ok(stats)
    }

    pub fn parse_stat(&mut self) -> Result<Node<Stat>, String> {
        self.start_node()?;

        let stat = match self.peek(0)? {
            Token::Name(_) | Token::LParens => {
                // Ambiguously an `Assignment` or a `FunctionCall`, so we have to rewind
                match self
                    .with_rewind(
                        |parser| {
                            let exp = parser.parse_var().map_err(|_| String::new())?;

                            // Eq or Comma denotes an assignment expression
                            if parser.next_is_in(&[Token::Comma, Token::Op(Op::Eq)]) {
                                Ok(exp)
                            } else {
                                // Otherwise we rewind, signalled by an empty string
                                // TODO: improve this
                                Err(String::new())
                            }
                        },
                        |e| e.is_empty(),
                    )?
                    .clone()
                {
                    // `Assignment`
                    Some(var) => {
                        self.fork_node()?;

                        let mut vars = vec![var];

                        while self.consume_a(Token::Comma) {
                            vars.push(self.parse_var()?)
                        }

                        self.expect(Op::Eq)?;

                        let exps = self.parse_list(Self::parse_exp)?;

                        Ok(self.produce_node(Assignment::new(vars, exps)).into())
                    }

                    // `FunctionCall`
                    None => {
                        let token = self.peek(0)?;

                        match self.parse_prefix_exp()?.inner {
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
                        self.fork_node()?;

                        let body = self.parse_block()?;

                        self.expect(Keyword::End)?;

                        Ok(self.produce_node(Do::new(body)).into())
                    }

                    // while exp do block end
                    Keyword::While => {
                        self.fork_node()?;

                        let cond = self.parse_exp()?;

                        self.expect(Keyword::Do)?;

                        let body = self.parse_block()?;

                        self.expect(Keyword::End)?;

                        Ok(self.produce_node(While::new(cond, body)).into())
                    }

                    // repeat block until exp
                    Keyword::Repeat => {
                        self.fork_node()?;

                        let body = self.parse_block()?;

                        self.expect(Keyword::Until)?;

                        let cond = self.parse_exp()?;

                        Ok(self.produce_node(RepeatUntil::new(body, cond)).into())
                    }

                    // if exp then block {elseif exp then block} [else block] end
                    Keyword::If => {
                        self.fork_node()?;

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

                        Ok(self
                            .produce_node(IfElse::new(cond, body, else_ifs, else_block))
                            .into())
                    }

                    Keyword::For => match self.peek(1)? {
                        // for Name `=´ exp `,´ exp [`,´ exp] do block end
                        Token::Op(Op::Eq) => {
                            self.fork_node()?;

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

                            Ok(self.produce_node(For::new(init, test, update, body)).into())
                        }

                        // for namelist in explist do block end
                        _ => {
                            self.fork_node()?;

                            let names = self.parse_list(Self::parse_name)?;

                            self.expect(Keyword::In)?;

                            let exps = self.parse_list(Self::parse_exp)?;

                            self.expect(Keyword::Do)?;

                            let body = self.parse_block()?;

                            self.expect(Keyword::End)?;

                            Ok(self.produce_node(ForIn::new(names, exps, body)).into())
                        }
                    },

                    // function funcname funcbody
                    Keyword::Function => {
                        self.fork_node()?;

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

                        let body = self.parse_function()?;

                        Ok(self
                            .produce_node(FunctionDef::new(false, name, body))
                            .into())
                    }

                    Keyword::Local => match self.peek(0)? {
                        // local function Name funcbody
                        Token::Keyword(Keyword::Function) => {
                            self.fork_node()?;

                            self.consume()?;

                            let name = self.parse_name()?;

                            let body = self.parse_function()?;

                            Ok(self.produce_node(FunctionDef::new(true, name, body)).into())
                        }

                        // local namelist [`=´ explist]
                        _ => {
                            self.fork_node()?;

                            let names = self.parse_list(Self::parse_name)?;

                            let init_exps = match self.consume_a(Op::Eq) {
                                true => Some(self.parse_list(Self::parse_exp)?),
                                false => None,
                            };

                            Ok(self.produce_node(VarDef::new(names, init_exps)).into())
                        }
                    },

                    _ => Err(format!("Unexpected `{:?}`, expected stat", keyword)),
                }
            }

            token => Err(format!("Unexpected `{:?}`, expected stat", token)),
        };

        Ok(self.produce_node(stat?))
    }

    fn parse_last_stat(&mut self) -> Result<Node<Stat>, String> {
        self.fork_node()?;
        let (token, _) = self.consume()?;

        let stat = match token {
            Token::Keyword(Keyword::Return) => {
                match self.with_rewind(
                    |parser| parser.parse_list(Self::parse_exp),
                    |e| e.ends_with("expected expression"),
                )? {
                    Some(exps) => {
                        self.fork_node()?;
                        Ok(self.produce_node(Return::new(exps)).into())
                    }
                    None => {
                        self.fork_node()?;
                        Ok(self.produce_node(Return::new(Vec::new())).into())
                    }
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

        Ok(self.produce_node(stat?))
    }

    pub fn parse_exp(&mut self) -> Result<Node<Exp>, String> {
        self.parse_exp_prec(Precedence::None)
    }

    fn parse_exp_prec(&mut self, min_precedence: Precedence) -> Result<Node<Exp>, String> {
        self.start_node()?;

        let mut lhs = {
            match get_nud_parselet(&self.peek(0)?) {
                Some(parselet) => {
                    self.fork_node()?;

                    let (token, _) = self.consume()?;

                    let exp = parselet.parse(self, token)?;

                    self.produce_node(exp)
                }

                None => self.parse_prefix_exp()?,
            }
        };

        while min_precedence < self.get_precedence() {
            let (token, _) = self.consume()?;

            lhs = match get_led_parselet(&token) {
                Some(parselet) => {
                    self.fork_node()?;

                    let exp = parselet.parse(self, lhs, token)?;

                    self.produce_node(exp)
                }

                None => return Err(format!("Unexpected `{:?}` in expression", token)),
            }
        }

        // Consume the latest node so it does not bleed into other areas
        self.consume_node()?;

        Ok(lhs)
    }

    fn parse_prefix_exp(&mut self) -> Result<Node<Exp>, String> {
        self.start_node()?;

        let mut lhs = {
            let (token, _) = self.consume()?;

            match get_prefix_nud_parselet(&token) {
                Some(parselet) => {
                    self.fork_node()?;

                    let exp = parselet.parse(self, token)?;

                    Ok(self.produce_node(exp))
                }

                None => Err(format!("Unexpected `{:?}`, expected expression", token)),
            }?
        };

        while let Ok(next) = self.peek(0) {
            if let Some(parselet) = get_prefix_led_parselet(&next) {
                self.fork_node()?;

                let (token, _) = self.consume()?;

                let exp = parselet.parse(self, lhs, token)?;

                lhs = self.produce_node(exp)
            } else {
                break;
            }
        }

        self.consume_node()?;

        Ok(lhs)
    }

    /// Parses a var, basically a more selective prefixexp
    fn parse_var(&mut self) -> Result<Node<Exp>, String> {
        let exp = self.parse_prefix_exp()?;

        match exp.inner {
            Exp::Index(_) => Ok(exp),

            Exp::Member(_) => Ok(exp),

            Exp::Ref(_) => Ok(exp),

            _ => Err("Unexpected prefixexp, expecting var".to_owned()),
        }
    }

    fn parse_function(&mut self) -> Result<Node<Function>, String> {
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

        Ok(self.produce_node(Function::new(params, body)))
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

    fn peek(&self, n: usize) -> Result<Token, String> {
        match self.tokens.iter().nth(n) {
            Some((token, _)) => Ok(token.clone()),

            None => Err("Unexpected EOF".to_owned()),
        }
    }

    fn consume(&mut self) -> Result<SpannedToken, String> {
        self.tokens
            .pop_front()
            .ok_or("Unexpected EOF".to_owned())
            .and_then(|token| {
                self.rewind_stack.push(token.clone());

                Ok(token)
            })
    }

    fn expect<E>(&mut self, expected: E) -> Result<(), String>
    where
        E: Into<Token>,
    {
        let (expected, (got, _)) = (expected.into(), self.consume()?);

        if got == expected {
            Ok(())
        } else {
            Err(format!("Unexpected `{:?}`, expected `{:?}`", got, expected))
        }
    }

    fn consume_a<E>(&mut self, expected: E) -> bool
    where
        E: Into<Token>,
    {
        return if self.next_is(expected) {
            // If `next_is` returns `true` but `consume` fails something very, very bad has happened
            self.consume().expect("Internal error");

            true
        } else {
            false
        };
    }

    fn next_is<E>(&mut self, expected: E) -> bool
    where
        E: Into<Token>,
    {
        match self.peek(0) {
            Ok(got) => got == expected.into(),

            Err(_) => false,
        }
    }

    fn next_is_in<P>(&mut self, possibilities: &[P]) -> bool
    where
        P: Into<Token> + Clone,
    {
        for possibility in possibilities {
            if self.next_is(possibility.clone().into()) {
                return true;
            }
        }

        false
    }

    fn with_rewind<T, F, C>(&mut self, func: F, can_rewind: C) -> Result<Option<T>, String>
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
            },
        }
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
                self.fork_node()?;

                let exp = TableConstructorParselet.parse(self, token)?;

                Ok(vec![self.produce_node(exp)])
            }

            // function"string"
            Token::Literal(Literal::String(arg)) => {
                self.fork_node()?;
                let inner_node = self.produce_node(arg);

                self.fork_node()?;
                let node = self.produce_node(Exp::String(inner_node));

                Ok(vec![node])
            }

            token => Err(format!("Unexpected {:?}, expected args", token)),
        }
    }

    /// Parse a name
    fn parse_name(&mut self) -> Result<String, String> {
        let (token, _) = self.consume()?;

        match token {
            Token::Name(name) => Ok(name),

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
        D: Into<Token>,
        P: Fn(&mut Parser) -> Result<T, String>,
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
        P: Fn(&mut Parser) -> Result<T, String>,
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

        Token::Name(_) => Some(&nud::NameParselet),

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
