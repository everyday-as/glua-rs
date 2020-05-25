use std::collections::VecDeque;

use crate::ast::*;
use crate::ast::stats::*;
use crate::lexer::*;
use crate::parser::parselets::{Led, led, Nud, nud};
use crate::parser::parselets::nud::{TableParselet, FunctionParselet};

mod parselets;

pub struct Parser {
    tokens: VecDeque<Token>,
    rewind_stack: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        // let mut cur_line = 1;

        Self {
            tokens: tokens
                .into_iter()
                .filter_map(|t| match t {
                    Token::Whitespace(lines) => {
                        // cur_line += lines;

                        None
                    }

                    Token::Comment(text) => {
                        // cur_line += text.chars().filter(|c| c == &'\n').count();

                        None
                    }

                    token => Some(token),
                })
                .collect(),
            rewind_stack: Vec::new(),
        }
    }

    pub fn parse_chunk(&mut self) -> Result<Block, String> {
        self.parse_block()
    }

    fn parse_block(&mut self) -> Result<Block, String> {
        let mut stats = Vec::new();

        // Rewind here, because Lua has SYNTACTICALLY ASCENDED THE MORTAL FUCKING PLANE
        while let Some(stat) = self.with_rewind(
            Self::parse_stat,
            |e| e == "Unexpected EOF" || e.ends_with("expected stat"))? {
            self.consume_a(Token::Semicolon);

            stats.push(stat);
        }

        if self.next_is_in(&[Keyword::Break, Keyword::Continue, Keyword::Return]) {
            stats.push(self.parse_last_stat()?)
        }

        Ok(stats)
    }

    pub fn parse_stat(&mut self) -> Result<Stat, String> {
        match self.peek(0)? {
            Token::Name(_) => {
                // Ambiguously an `Assignment` or a `FunctionCall`, so we have to rewind
                match self.with_rewind(|parser| {
                    let exp = parser.parse_var_exp()?;

                    // Eq or Comma denotes an assignment expression
                    if parser.next_is_in(&[Token::Comma, Token::Op(Op::Eq)]) {
                        Ok(exp)
                    } else {
                        // Otherwise we rewind, signalled by an empty string
                        // TODO: improve this
                        Err(String::new())
                    }
                }, |e| e.is_empty())? {
                    // `Assignment`
                    Some(var) => {
                        let mut vars = vec![var];

                        while self.consume_a(Token::Comma) {
                            vars.push(self.parse_var_exp()?)
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

                            _ => Err(format!("Unexpected `{:?}`, expected functioncall", token))
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

                        self.expect(Keyword::End)?;

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
                            false => None
                        };

                        self.expect(Keyword::End)?;

                        Ok(IfElse::new(cond, else_ifs, else_block).into())
                    }

                    Keyword::For => match self.peek(1)? {
                        // for Name `=´ exp `,´ exp [`,´ exp] do block end
                        Token::Op(Op::Eq) => {
                            let init = {
                                let name = self.parse_name()?;

                                self.expect(Op::Eq)?;

                                let exp = self.parse_exp()?;

                                Assignment::new(vec![Exp::Ref(name)], vec![exp])
                            };

                            self.expect(Token::Comma)?;

                            let test = self.parse_exp()?;

                            let update = match self.consume_a(Token::Comma) {
                                true => Some(self.parse_exp()?),
                                false => None
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
                    }

                    // function funcname funcbody
                    Keyword::Function => {
                        let name = {
                            let parts = self.parse_delimited(
                                Op::Dot,
                                Self::parse_name,
                                |token| [Token::LParens, Token::Op(Op::Colon)].contains(&token),
                            ).unwrap();

                            let mut name = parts.join(".");

                            if self.consume_a(Op::Colon) {
                                name.push(':');
                                name.push_str(&self.parse_name()?)
                            }

                            name
                        };

                        let body = self.parse_function()?;

                        Ok(FunctionDef::new(false, name, body).into())
                    }

                    Keyword::Local => match self.peek(0)? {
                        // local function Name funcbody
                        Token::Keyword(Keyword::Function) => {
                            self.consume()?;

                            let name = self.parse_name()?;

                            let body = self.parse_function()?;

                            Ok(FunctionDef::new(true, name, body).into())
                        }

                        // local namelist [`=´ explist]
                        _ => {
                            let names = self.parse_list(Self::parse_name)?;

                            let init_exps = match self.consume_a(Op::Eq) {
                                true => Some(self.parse_list(Self::parse_exp)?),
                                false => None
                            };

                            Ok(VarDef::new(names, init_exps).into())
                        }
                    }

                    _ => Err(format!("Unexpected `{:?}`, expected stat", keyword))
                }
            }

            token => Err(format!("Unexpected `{:?}`, expected stat", token))
        }
    }

    fn parse_last_stat(&mut self) -> Result<Stat, String> {
        match self.consume()? {
            Token::Keyword(Keyword::Return) => {
                match self.with_rewind(
                    |parser| parser.parse_list(Self::parse_exp),
                    |e| e.ends_with("expected exp"),
                )? {
                    Some(exps) => Ok(Return::new(exps).into()),
                    None => Ok(Return::new(Vec::new()).into())
                }
            }

            Token::Keyword(Keyword::Break) => Ok(Stat::Break),

            // GMod specific
            Token::Keyword(Keyword::Continue) => Ok(Stat::Continue),

            token => Err(
                format!("Unexpected `{:?}`, expected return, continue or break", token)
            )
        }
    }

    pub fn parse_exp(&mut self) -> Result<Exp, String> {
        self.parse_exp_prec(Precedence::None)
    }

    fn parse_exp_prec(&mut self, min_precedence: Precedence) -> Result<Exp, String> {
        let mut lhs = {
            match get_nud_parselet(self.peek(0)?) {
                Some(parselet) => {
                    let token = self.consume()?;

                    parselet.parse(self, token)?
                }

                None => self.parse_prefix_exp()?
            }
        };

        while min_precedence < self.get_precedence() {
            let token = self.consume()?;

            lhs = match get_led_parselet(token.clone()) {
                Some(parselet) => parselet.parse(self, lhs, token)?,

                None => return Err(format!("Unexpected `{:?}` in expression", token))
            }
        }

        Ok(lhs)
    }

    fn parse_prefix_exp(&mut self) -> Result<Exp, String> {
        let mut lhs = {
            match get_prefix_nud_parselet(self.peek(0)?) {
                Some(parselet) => {
                    let token = self.consume()?;

                    parselet.parse(self, token)?
                }

                None => self.parse_var_exp()?
            }
        };

        while let Ok(next) = self.peek(0) {
            if let Some(parselet) = get_prefix_led_parselet(next) {
                let token = self.consume()?;

                lhs = parselet.parse(self, lhs, token)?
            } else {
                break;
            }
        }

        Ok(lhs)
    }

    fn parse_var_exp(&mut self) -> Result<Exp, String> {
        let mut lhs = {
            let token = self.peek(0)?;

            match get_var_nud_parselet(token.clone()) {
                Some(parselet) => {
                    let token = self.consume()?;

                    parselet.parse(self, token)?
                }

                None => return Err(format!("Unexpected `{:?}`, expected expression", token))
            }
        };

        while let Ok(next) = self.peek(0) {
            if let Some(parselet) = get_var_led_parselet(next) {
                let token = self.consume()?;

                lhs = parselet.parse(self, lhs, token)?
            } else {
                break;
            }
        }

        Ok(lhs)
    }

    fn parse_function(&mut self) -> Result<Function, String> {
        self.expect(Token::LParens)?;

        let mut params = self.parse_delimited(
            Token::Comma,
            Parser::parse_name,
            |token| Token::Ellipsis == token || Token::RParens == token,
        )?;

        if self.consume_a(Token::Ellipsis) {
            params.push("...".to_owned());
        }

        self.expect(Token::RParens)?;

        let body = self.parse_block()?;

        self.expect(Keyword::End);

        Ok(Function::new(params, body).into())
    }

    // <Helpers>
    fn get_precedence(&self) -> Precedence {
        match self.peek(0) {
            Ok(token) => match get_led_parselet(token.clone()) {
                Some(parselet) => parselet.get_precedence(),

                None => match get_prefix_led_parselet(token) {
                    Some(parselet) => parselet.get_precedence(),

                    None => Precedence::None
                }
            }

            Err(_) => Precedence::None
        }
    }

    fn peek(&self, n: usize) -> Result<Token, String> {
        match self.tokens.iter().nth(n) {
            Some(token) => Ok(token.clone()),

            None => Err("Unexpected EOF".to_owned())
        }
    }

    fn consume(&mut self) -> Result<Token, String> {
        self.tokens.pop_front()
            .ok_or("Unexpected EOF".to_owned())
            .and_then(|token| {
                self.rewind_stack.push(token.clone());

                Ok(token)
            })
    }

    fn expect<E>(&mut self, expected: E) -> Result<(), String> where E: Into<Token> {
        let (expected, got) = (expected.into(), self.consume()?);

        if got == expected {
            Ok(())
        } else {
            Err(format!("Unexpected `{:?}`, expected `{:?}`", got, expected))
        }
    }

    fn consume_a<E>(&mut self, expected: E) -> bool where E: Into<Token> {
        return if self.next_is(expected) {
            // If `next_is` returns `true` but `consume` fails something very, very bad has happened
            self.consume().expect("Internal error");

            true
        } else {
            false
        };
    }

    fn next_is<E>(&mut self, expected: E) -> bool where E: Into<Token> {
        match self.peek(0) {
            Ok(got) => got == expected.into(),

            Err(_) => false
        }
    }

    fn next_is_in<P>(&mut self, possibilities: &[P]) -> bool where P: Into<Token> + Clone {
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
            C: FnOnce(&str) -> bool
    {
        let rewind_to = self.rewind_stack.len();

        match func(self) {
            Ok(result) => Ok(Some(result)),
            Err(err) => match can_rewind(&err) {
                true => {
                    if self.rewind_stack.len() > rewind_to {
                        println!("Rewinding: {:?} ({}<-{})", err, rewind_to, self.rewind_stack.len());
                    }

                    while self.rewind_stack.len() > rewind_to {
                        self.tokens.push_front(self.rewind_stack.pop().unwrap());
                    }

                    Ok(None)
                }

                false => Err(err)
            }
        }
    }
    // </Helpers>

    // <Parse Helpers>
    /// Parse function / method arguments
    fn parse_args(&mut self, token: Token) -> Result<Vec<Exp>, String> {
        match token {
            // function(arg, arg2)
            Token::LParens => match self.consume_a(Token::RParens) {
                true => Ok(Vec::new()),
                false => {
                    let args = self.parse_list(Self::parse_exp)?;

                    self.expect(Token::RParens)?;

                    Ok(args)
                }
            }

            // function{ table }
            Token::LBrace => {
                Ok(vec![TableParselet.parse(self, token)?])
            }

            // function"string"
            Token::Literal(Literal::String(arg)) => {
                Ok(vec![Exp::String(arg)])
            }

            token => Err(format!("Unexpected {:?}, expected args", token))
        }
    }

    /// Parse a name
    fn parse_name(&mut self) -> Result<String, String> {
        match self.consume()? {
            Token::Name(name) => Ok(name),

            token => Err(format!("Unexpected `{:?}`, expected name", token))
        }
    }

    fn parse_delimited<T, D, P, IE>(&mut self, delim: D, parse: P, is_end: IE) -> Result<Vec<T>, String>
        where
            D: Into<Token>,
            P: Fn(&mut Parser) -> Result<T, String>,
            IE: Fn(Token) -> bool
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
        where P: Fn(&mut Parser) -> Result<T, String>
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

fn get_nud_parselet(token: Token) -> Option<&'static dyn Nud> {
    match token {
        Token::Ellipsis => Some(&nud::EllipsisParselet),

        Token::Keyword(Keyword::Function) => Some(&nud::FunctionParselet),

        Token::LBrace => Some(&nud::TableParselet),

        Token::Literal(_) => Some(&nud::LiteralParselet),

        Token::Op(Op::Len) | Token::Op(Op::Not)
        | Token::Op(Op::Sub) => Some(&nud::UnaryParselet),

        _ => None
    }
}

fn get_led_parselet(token: Token) -> Option<&'static dyn Led> {
    match token {
        Token::Op(Op::Exp) => Some(&led::ExponentiationParselet),

        Token::Op(Op::Mod) | Token::Op(Op::Mul)
        | Token::Op(Op::Div) => Some(&led::MultiplicativeParselet),

        Token::Op(Op::Add) | Token::Op(Op::Sub) => Some(&led::AdditiveParselet),

        Token::Op(Op::DotDot) => Some(&led::ConcatParselet),

        Token::Op(Op::Lt) | Token::Op(Op::Gt)
        | Token::Op(Op::LtEq) | Token::Op(Op::GtEq)
        | Token::Op(Op::Ne) | Token::Op(Op::EqEq) => Some(&led::ComparativeParselet),

        Token::Op(Op::And) => Some(&led::AndParselet),

        Token::Op(Op::Or) => Some(&led::OrParselet),

        _ => None
    }
}

fn get_prefix_nud_parselet(token: Token) -> Option<&'static dyn Nud> {
    match token {
        Token::LParens => Some(&nud::ParensParselet),

        _ => None
    }
}

fn get_prefix_led_parselet(token: Token) -> Option<&'static dyn Led> {
    match token {
        Token::LParens | Token::LBrace
        | Token::Literal(Literal::String(_)) => Some(&led::FunctionCallParselet),

        Token::Op(Op::Colon) => Some(&led::MethodCallParselet),

        _ => None
    }
}

fn get_var_nud_parselet(token: Token) -> Option<&'static dyn Nud> {
    match token {
        Token::Name(_) => Some(&nud::NameParselet),

        _ => None
    }
}

fn get_var_led_parselet(token: Token) -> Option<&'static dyn Led> {
    match token {
        Token::LBracket | Token::Op(Op::Dot) => Some(&led::AccessParselet),

        _ => None
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
enum Precedence {
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
