use std::fmt::Debug;

use bumpalo::{
    collections::{String as BumpString, Vec as BumpVec},
    Bump,
};
use logos::Logos;
pub use logos::Span;

use crate::{
    ast::{exps::*, node::Node, stats::*, Exp, Stat, *},
    lexer::*,
    parser::{
        error::{Error, Expectation},
        parselets::{led, nud, nud::TableConstructorParselet, Led, Nud},
    },
};

mod error;
mod parselets;

pub type Result<'a, T, E = Error<'a>> = std::result::Result<T, E>;

pub type SpannedToken<'a> = (Token<'a>, Span);

pub struct Parser<'a> {
    bump: &'a Bump,
    tokens: &'a [SpannedToken<'a>],
    pos: usize,
}

enum Rewind<'a> {
    Rewind,
    Abort(Error<'a>),
}

impl<'a> Parser<'a> {
    pub fn lex(source: &'a str, bump: &'a Bump) -> Result<'a, Vec<SpannedToken<'a>>> {
        Token::lexer_with_extras(source, bump)
            .spanned()
            .filter_map(|(res, span)| match res {
                Ok(Token::Comment(_)) => None,
                Ok(token) => Some(Ok((token, span))),
                Err(_) => Some(Err(Error::Lexer(span))),
            })
            .collect()
    }

    pub fn new_in(tokens: &'a [SpannedToken<'a>], bump: &'a Bump) -> Self {
        Self {
            tokens,
            bump,
            pos: 0,
        }
    }

    pub fn parse_chunk(&mut self) -> Result<'a, Block<'a>> {
        let block = self.parse_block()?;

        match self.tokens.get(self.pos) {
            None => Ok(block),
            Some((token, span)) => Err(Error::unexpected_token(span, Expectation::Eof, *token)),
        }
    }

    fn parse_block(&mut self) -> Result<'a, Block<'a>> {
        let mut stats = BumpVec::new_in(self.bump);

        // Rewind here, because Lua has SYNTACTICALLY ASCENDED THE MORTAL FUCKING PLANE
        while let Some(stat) = self.with_rewind(|p| match p.node(Self::parse_stat) {
            Err(
                Error::UnexpectedEof { .. }
                | Error::UnexpectedToken {
                    expected: Some(Expectation::Stat),
                    ..
                },
            ) => Err(Rewind::Rewind),
            res => res.map_err(Rewind::Abort),
        })? {
            self.consume_a(Token::Semicolon);

            stats.push(stat);
        }

        if self.next_is_in([Keyword::Break, Keyword::Continue, Keyword::Return]) {
            stats.push(self.node(|p| p.parse_last_stat())?);

            self.consume_a(Token::Semicolon);
        }

        Ok(stats.into_bump_slice())
    }

    pub fn parse_stat(&mut self) -> Result<'a, Stat<'a>> {
        // We should only match `goto` as a potential expression if it's _not_ followed by a name,
        // as that would make it a valid `goto` statement.
        let name_after = matches!(self.peek(1), Ok(Token::Name(_)));

        match (self.peek(0)?, name_after) {
            (Token::Keyword(Keyword::Goto), false) | (Token::Name(_), _) | (Token::LParens, _) => {
                // Ambiguously an `Assignment` or a `FunctionCall`, so we have to rewind
                match self.with_rewind(|parser| {
                    let exp = parser.node(Self::parse_var).map_err(|_| Rewind::Rewind)?;

                    // Eq or Comma denotes an assignment expression
                    if parser.next_is_in([Token::Comma, Token::Op(Op::Eq)]) {
                        Ok(exp)
                    } else {
                        Err(Rewind::Rewind)
                    }
                })? {
                    // `Assignment`
                    Some(var) => {
                        let mut vars = bumpalo::vec![in self.bump; var];

                        while self.consume_a(Token::Comma) {
                            vars.push(self.node(Self::parse_var)?)
                        }

                        self.expect(Op::Eq)?;

                        let exps = self.parse_list(|p| p.node(Self::parse_exp))?;

                        Ok(Assignment::new(vars.into_bump_slice(), exps.into_bump_slice()).into())
                    }

                    // `FunctionCall`
                    None => match self.parse_prefix_exp()? {
                        Exp::FunctionCall(call) => Ok(Stat::FunctionCall(call)),

                        Exp::MethodCall(call) => Ok(Stat::MethodCall(call)),

                        _ => Err(Error::unexpected_token(
                            self.span()?,
                            Expectation::FunctionCall,
                            *self.peek(0)?,
                        )),
                    },
                }
            }

            (Token::Keyword(keyword), _) => {
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
                        let cond = self.node(Self::parse_exp)?;

                        self.expect(Keyword::Do)?;

                        let body = self.parse_block()?;

                        self.expect(Keyword::End)?;

                        Ok(While::new(cond, body).into())
                    }

                    // repeat block until exp
                    Keyword::Repeat => {
                        let body = self.parse_block()?;

                        self.expect(Keyword::Until)?;

                        let cond = self.node(Self::parse_exp)?;

                        Ok(RepeatUntil::new(body, cond).into())
                    }

                    // if exp then block {elseif exp then block} [else block] end
                    Keyword::If => {
                        let cond = self.node(Self::parse_exp)?;

                        self.expect(Keyword::Then)?;

                        let body = self.parse_block()?;

                        let mut else_ifs = BumpVec::new_in(self.bump);

                        while self.consume_a(Keyword::ElseIf) {
                            let cond = self.node(Self::parse_exp)?;

                            self.expect(Keyword::Then)?;

                            let body = self.parse_block()?;

                            else_ifs.push((cond, body));
                        }

                        let else_block = match self.consume_a(Keyword::Else) {
                            true => Some(self.parse_block()?),
                            false => None,
                        };

                        self.expect(Keyword::End)?;

                        Ok(IfElse::new(cond, body, else_ifs.into_bump_slice(), else_block).into())
                    }

                    Keyword::For => match self.peek(1)? {
                        // for Name `=´ exp `,´ exp [`,´ exp] do block end
                        Token::Op(Op::Eq) => {
                            let init = {
                                let name = self.parse_name()?;

                                self.expect(Op::Eq)?;

                                let exp = self.node(Self::parse_exp)?;

                                (name, exp)
                            };

                            self.expect(Token::Comma)?;

                            let test = self.node(Self::parse_exp)?;

                            let update = match self.consume_a(Token::Comma) {
                                true => Some(self.node(Self::parse_exp)?),
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

                            let exps = self.parse_list(|p| p.node(Self::parse_exp))?;

                            self.expect(Keyword::Do)?;

                            let body = self.parse_block()?;

                            self.expect(Keyword::End)?;

                            Ok(
                                ForIn::new(names.into_bump_slice(), exps.into_bump_slice(), body)
                                    .into(),
                            )
                        }
                    },

                    // function funcname funcbody
                    Keyword::Function => {
                        let name = {
                            let mut len = 0;

                            let parts = self.parse_delimited(
                                Op::Dot,
                                |p| {
                                    let part = p.parse_name()?;

                                    len += part.len();

                                    Ok(part)
                                },
                                |token| [Token::LParens, Token::Op(Op::Colon)].contains(token),
                            )?;

                            let mut name = BumpString::with_capacity_in(len, self.bump);

                            parts.iter().for_each(|part| name.push_str(part));

                            if self.consume_a(Op::Colon) {
                                let part = self.parse_name()?;

                                name.reserve_exact(part.len() + 1);

                                name.push(':');
                                name.push_str(part)
                            }

                            name
                        };

                        let body = self.node(|p| p.parse_function())?;

                        Ok(FunctionDef::new(false, name.into_bump_str(), body).into())
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
                                true => Some(self.parse_list(|p| p.node(Self::parse_exp))?),
                                false => None,
                            };

                            Ok(VarDef::new(
                                names.into_bump_slice(),
                                init_exps.map(BumpVec::into_bump_slice),
                            )
                            .into())
                        }
                    },

                    Keyword::Goto => match self.peek(0)? {
                        // Goto must be followed by a Token::Name
                        Token::Name(label) => {
                            // goto Name
                            self.consume()?;

                            Ok(Goto::new(label).into())
                        }

                        token => Err(Error::unexpected_token(
                            self.span()?,
                            Expectation::Name,
                            *token,
                        )),
                    },

                    _ => Err(Error::unexpected_token(
                        self.span()?,
                        Expectation::Stat,
                        *keyword,
                    )),
                }
            }

            (Token::Label(name), _) => {
                self.consume()?;

                Ok(Label::new(name).into())
            }

            (token, _) => Err(Error::unexpected_token(
                self.span()?,
                Expectation::Stat,
                *token,
            )),
        }
    }

    fn parse_last_stat(&mut self) -> Result<'a, Stat<'a>> {
        match self.consume()? {
            Token::Keyword(Keyword::Return) => {
                match self.with_rewind(|parser| {
                    let res = parser.parse_list(|p| p.node(Self::parse_exp));

                    match res {
                        Err(Error::UnexpectedToken {
                            expected: Some(Expectation::Expression),
                            ..
                        }) => Err(Rewind::Rewind),
                        _ => res.map_err(Rewind::Abort),
                    }
                })? {
                    Some(exps) => Ok(Return::new(exps.into_bump_slice()).into()),
                    None => Ok(Return::new(&[]).into()),
                }
            }

            Token::Keyword(Keyword::Break) => Ok(Stat::Break),

            // GMod specific
            Token::Keyword(Keyword::Continue) => Ok(Stat::Continue),

            token => Err(Error::unexpected_token(
                self.span()?,
                Expectation::tokens([Keyword::Return, Keyword::Continue, Keyword::Break]),
                *token,
            )),
        }
    }

    pub fn parse_exp(&mut self) -> Result<'a, Exp<'a>> {
        self.parse_exp_prec(Precedence::None)
    }

    fn parse_exp_prec(&mut self, min_precedence: Precedence) -> Result<'a, Exp<'a>> {
        let mut lhs = self.stack_node(|p| match get_nud_parselet(p.peek(0)?) {
            Some(parselet) => {
                let token = p.consume()?;

                parselet.parse(p, *token)
            }

            None => p.parse_prefix_exp(),
        })?;

        while min_precedence < self.get_precedence() {
            let token = self.consume()?;

            lhs = match get_led_parselet(token) {
                Some(parselet) => {
                    self.stack_node(move |p| parselet.parse(p, p.alloc_node(lhs), *token))?
                }

                None => return Err(Error::unexpected_token(self.span()?, None, *token)),
            }
        }

        Ok(lhs.into_inner())
    }

    fn parse_prefix_exp(&mut self) -> Result<'a, Exp<'a>> {
        let mut lhs = self.stack_node(|p| {
            let token = p.consume()?;

            match get_prefix_nud_parselet(token) {
                Some(parselet) => parselet.parse(p, *token),

                None => Err(Error::unexpected_token(
                    p.last_span()?,
                    Expectation::Expression,
                    *token,
                )),
            }
        })?;

        while let Ok(next) = self.peek(0) {
            if let Some(parselet) = get_prefix_led_parselet(next) {
                lhs = self.stack_node(|p| {
                    let token = p.consume()?;

                    parselet.parse(p, p.alloc_node(lhs), *token)
                })?;
            } else {
                break;
            }
        }

        Ok(lhs.into_inner())
    }

    /// Parses a var, basically a more selective prefixexp
    fn parse_var(&mut self) -> Result<'a, Exp<'a>> {
        let exp = self.parse_prefix_exp()?;

        match exp {
            Exp::Index(_) => Ok(exp),

            Exp::Member(_) => Ok(exp),

            Exp::Ref(_) => Ok(exp),

            _ => Err(Error::unexpected_expression(
                self.span()?,
                Expectation::Var,
                exp,
            )),
        }
    }

    fn parse_function(&mut self) -> Result<'a, Function<'a>> {
        self.expect(Token::LParens)?;

        let mut params = self.parse_delimited(Token::Comma, Self::parse_name, |token| {
            matches!(token, Token::Ellipsis | Token::RParens)
        })?;

        if self.consume_a(Token::Ellipsis) {
            params.push("...");
        }

        self.expect(Token::RParens)?;

        let body = self.parse_block()?;

        self.expect(Keyword::End)?;

        Ok(Function::new(params.into_bump_slice(), body))
    }

    // <Helpers>
    fn get_precedence(&self) -> Precedence {
        match self.peek(0) {
            Ok(token) => match get_led_parselet(token) {
                Some(parselet) => parselet.get_precedence(),

                None => match get_prefix_led_parselet(token) {
                    Some(parselet) => parselet.get_precedence(),

                    None => Precedence::None,
                },
            },

            Err(_) => Precedence::None,
        }
    }

    fn peek(&self, n: usize) -> Result<'a, &'a Token<'a>> {
        match self.tokens.get(self.pos + n) {
            Some((token, _)) => Ok(token),

            None => Err(Error::unexpected_eof(None::<Expectation>)),
        }
    }

    fn consume(&mut self) -> Result<'a, &'a Token<'a>> {
        self.tokens
            .get(self.pos)
            .ok_or(Error::unexpected_eof(None::<Expectation>))
            .map(|(token, _)| {
                self.pos += 1;

                token
            })
    }

    fn expect<E>(&mut self, expected: E) -> Result<'a, ()>
    where
        E: Debug + Into<Option<Expectation<'a>>> + PartialEq<Token<'a>>,
    {
        let got = match self.consume() {
            Err(Error::UnexpectedEof { .. }) => return Err(Error::unexpected_eof(expected)),

            Ok(token) => token,

            _ => unreachable!(),
        };

        if expected.eq(got) {
            Ok(())
        } else {
            Err(Error::unexpected_token(self.span()?, expected, *got))
        }
    }

    fn consume_a(&mut self, expected: impl PartialEq<Token<'a>>) -> bool {
        let consume = self.next_is(expected);

        if consume {
            let _ = self.consume();
        }

        consume
    }

    fn next_is(&mut self, expected: impl PartialEq<Token<'a>>) -> bool {
        self.peek(0).map(|got| expected.eq(got)).unwrap_or(false)
    }

    fn next_is_in<P>(&mut self, possibilities: impl IntoIterator<Item = P>) -> bool
    where
        P: PartialEq<Token<'a>>,
    {
        possibilities.into_iter().any(|p| self.next_is(p))
    }

    fn with_rewind<T, F>(&mut self, func: F) -> Result<'a, Option<T>>
    where
        F: FnOnce(&mut Parser<'a>) -> Result<'a, T, Rewind<'a>>,
    {
        let rewind_to = self.pos;

        match func(self) {
            Ok(result) => Ok(Some(result)),
            Err(err) => match err {
                Rewind::Rewind => {
                    self.pos = rewind_to;

                    Ok(None)
                }

                Rewind::Abort(err) => Err(err),
            },
        }
    }

    /// Produce and allocate a node on the bump heap
    fn node<T>(&mut self, f: impl FnOnce(&mut Self) -> Result<'a, T>) -> Result<'a, Node<&'a T>> {
        let node = self.stack_node(f)?;

        Ok(self.alloc_node(node))
    }

    /// Produce a stack-allocated node
    fn stack_node<T>(&mut self, f: impl FnOnce(&mut Self) -> Result<'a, T>) -> Result<'a, Node<T>> {
        let start = self.span()?.start;

        let inner = f(self)?;

        let span = start..self.last_span()?.end;

        Ok(Node::new(span, inner))
    }

    /// Allocate a stack node on the bump heap
    fn alloc_node<T>(&self, node: Node<T>) -> Node<&'a T> {
        Node::map(node, |value| self.bump.alloc(value) as &_)
    }

    fn span(&self) -> Result<'a, &Span> {
        self.tokens
            .get(self.pos)
            .map(|(_, span)| span)
            .ok_or(Error::unexpected_eof(None))
    }

    fn last_span(&self) -> Result<'a, &Span> {
        self.tokens
            .get(self.pos.wrapping_sub(1))
            .map(|(_, span)| span)
            .ok_or(Error::unexpected_eof(None))
    }
    // </Helpers>

    // <Parse Helpers>
    /// Parse function / method arguments
    fn parse_args(&mut self, token: Token<'a>) -> Result<'a, BumpVec<'a, Node<&'a Exp<'a>>>> {
        match token {
            // function(arg, arg2)
            Token::LParens => {
                if self.consume_a(Token::RParens) {
                    Ok(BumpVec::new_in(self.bump))
                } else {
                    let args = self.parse_list(|p| p.node(Self::parse_exp))?;

                    self.expect(Token::RParens)?;

                    Ok(args)
                }
            }

            // function{ table }
            Token::LBrace => {
                let start = self.last_span()?.start;

                let inner = TableConstructorParselet.parse(self, token)?;

                let span = start..self.last_span()?.end;

                Ok(bumpalo::vec![in self.bump; self.alloc_node(Node::new(span, inner))])
            }

            // function"string"
            Token::Literal(Literal::String(arg)) => Ok(bumpalo::vec![
                in self.bump;
                self.alloc_node(Node::new(self.last_span()?.clone(), Exp::String(arg)))
            ]),

            token => Err(Error::unexpected_token(
                self.span()?,
                Expectation::Args,
                token,
            )),
        }
    }

    /// Parse a name
    fn parse_name(&mut self) -> Result<'a, &'a str> {
        let token = self.consume()?;

        match token {
            Token::Name(name) => Ok(name),
            Token::Keyword(Keyword::Goto) => Ok("goto"),

            _ => Err(Error::unexpected_token(
                self.span()?,
                Expectation::Name,
                *token,
            )),
        }
    }

    fn parse_delimited<T, D, P, IE>(
        &mut self,
        delim: D,
        mut parse: P,
        is_end: IE,
    ) -> Result<'a, BumpVec<'a, T>>
    where
        T: 'a,
        D: Into<Token<'a>>,
        P: FnMut(&mut Parser<'a>) -> Result<'a, T>,
        IE: Fn(&Token) -> bool,
    {
        let delim = delim.into();

        let mut items = BumpVec::new_in(self.bump);

        while !is_end(self.peek(0)?) {
            items.push(parse(self)?);

            if !is_end(self.peek(0)?) {
                self.expect(delim)?;
            }
        }

        Ok(items)
    }

    fn parse_list<T, P>(&mut self, parse: P) -> Result<'a, BumpVec<'a, T>>
    where
        P: Fn(&mut Parser<'a>) -> Result<'a, T>,
    {
        let mut items = BumpVec::new_in(self.bump);

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

const fn get_nud_parselet(token: &Token) -> Option<&'static dyn Nud> {
    match token {
        Token::Ellipsis => Some(&nud::EllipsisParselet),

        Token::Keyword(Keyword::Function) => Some(&nud::FunctionParselet),

        Token::LBrace => Some(&nud::TableConstructorParselet),

        Token::Literal(_) => Some(&nud::LiteralParselet),

        Token::Op(Op::Len) | Token::Op(Op::Not) | Token::Op(Op::Sub) => Some(&nud::UnaryParselet),

        _ => None,
    }
}

const fn get_led_parselet(token: &Token) -> Option<&'static dyn Led> {
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

const fn get_prefix_nud_parselet(token: &Token) -> Option<&'static dyn Nud> {
    match token {
        Token::LParens => Some(&nud::ParensParselet),

        Token::Keyword(Keyword::Goto) | Token::Name(_) => Some(&nud::NameParselet),

        _ => None,
    }
}

const fn get_prefix_led_parselet(token: &Token) -> Option<&'static dyn Led> {
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
