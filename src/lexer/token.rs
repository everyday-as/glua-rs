use crate::lexer::{Keyword, Literal, Op};

use logos::{Logos, Lexer};
use logos::internal::{CallbackResult, LexerInternal};

#[derive(Clone, Debug, Logos, PartialEq)]
pub enum Token {
    #[token(",")]
    Comma,
    #[regex(r"--[^\[][^\n]*", |lex| lex.slice().to_string())]
    #[regex(r"--\[(=*)\[", parse_multi_line)]
    // GMod specific
    #[regex("//[^\n]*", |lex| lex.slice().to_string())]
    #[regex(r"/\*(~(.*\*/.*))\*/", |lex| lex.slice().to_string())]
    Comment(String),
    #[token("...")]
    Ellipsis,
    #[token("break", |_| Keyword::Break)]
    #[token("do", |_| Keyword::Do)]
    #[token("else", |_| Keyword::Else)]
    #[token("elseif", |_| Keyword::ElseIf)]
    #[token("end", |_| Keyword::End)]
    #[token("for", |_| Keyword::For)]
    #[token("function", |_| Keyword::Function)]
    #[token("if", |_| Keyword::If)]
    #[token("in", |_| Keyword::In)]
    #[token("local", |_| Keyword::Local)]
    #[token("repeat", |_| Keyword::Repeat)]
    #[token("return", |_| Keyword::Return)]
    #[token("then", |_| Keyword::Then)]
    #[token("until", |_| Keyword::Until)]
    #[token("while", |_| Keyword::While)]
    // GMod specific
    #[token("continue", |_| Keyword::Continue)]
    Keyword(Keyword),
    #[token("{")]
    LBrace,
    #[token("[")]
    LBracket,
    #[token("false", |_| Literal::Bool(false))]
    #[token("true", |_| Literal::Bool(true))]
    #[token("nil", |_| Literal::Nil)]
    #[regex(r"-?[0-9]+(\.[0-9]+)?(e(\+|-)?[0-9]+)?", |lex| {
        Some(Literal::Number(lex.slice().parse().ok()?))
    })]
    #[regex(r#""([^"\\\n]|\\.)*""#, parse_string)]
    #[regex(r"'([^'\\\n]|\\.)*'", parse_string)]
    #[regex(r"\[(=*)\[", |lex| parse_multi_line(lex).map(Literal::String))]
    Literal(Literal),
    #[token("(")]
    LParens,
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_owned())]
    Name(String),
    #[token("+", |_| Op::Add)]
    #[token("and", |_| Op::And)]
    #[token(":", |_| Op::Colon)]
    #[token("/", |_| Op::Div)]
    #[token(".", |_| Op::Dot)]
    #[token("..", |_| Op::DotDot)]
    #[token("=", |_| Op::Eq)]
    #[token("==", |_| Op::EqEq)]
    #[token("^", |_| Op::Exp)]
    #[token(">", |_| Op::Gt)]
    #[token(">=", |_| Op::GtEq)]
    #[token("#", |_| Op::Len)]
    #[token("<", |_| Op::Lt)]
    #[token("<=", |_| Op::LtEq)]
    #[token("%", |_| Op::Mod)]
    #[token("*", |_| Op::Mul)]
    #[token("~=", |_| Op::Ne)]
    #[token("not", |_| Op::Not)]
    #[token("or", |_| Op::Or)]
    #[token("-", |_| Op::Sub)]
    // GMod specific
    #[token("&&", |_| Op::And)]
    #[token("||", |_| Op::Or)]
    #[token("!", |_| Op::Not)]
    #[token("!=", |_| Op::Ne)]
    Op(Op),
    #[token("}")]
    RBrace,
    #[token("]")]
    RBracket,
    #[token(")")]
    RParens,
    #[token(";")]
    Semicolon,
    #[regex(r"[ \t\r\n\f]+", |lex| Some(lex.slice().chars().filter(|c| c == &'\n').count()))]
    Whitespace(usize),

    #[error]
    Error
}

#[derive(PartialEq)]
enum StringType {
    DoubleQuoted,
    MultiLine,
    SingleQuoted
}

fn parse_string(lexer: &mut Lexer<Token>) -> Option<Literal> {
    let slice = lexer.slice();

    let pad = match slice.chars().nth(0).unwrap() {
        '[' => 2,
        _ => 1
    };

    let mut value = String::with_capacity(slice.len());

    let mut escaped = false;

    for ch in slice[pad..slice.len() - pad].chars() {
        match ch {
            '\\' if !escaped => escaped = true,

            _ => {
                value.push(ch);

                escaped = false
            }
        }
    }

    Some(Literal::String(value))
}

fn parse_multi_line(lexer: &mut Lexer<Token>) -> Option<String> {
    // Offset past comment dashes
    let offset = match "-" == &lexer.slice()[0..1] {
        true => 2,
        false => 0
    };

    let len = lexer.slice().len();

    let closing = {
        let mut buf = String::with_capacity(len);

        buf.push(']');

        buf.push_str(&lexer.slice()[(offset + 1)..(len - 1)]);

        buf.push(']');

        buf
    };

    lexer.remainder()
        .find(&closing)
        .map(|i| lexer.bump(i + closing.len()))
        .map(|_| lexer.slice()[len..lexer.slice().len() - len].to_owned())
}
