use crate::lexer::{Keyword, Literal, Op};

use logos::{Lexer, Logos};

#[derive(Clone, Debug, Logos, PartialEq)]
pub enum Token {
    #[token(",")]
    Comma,
    #[regex(r"--([^\[][^\n]*)?", | lex | lex.slice().to_string())]
    #[regex(r"--\[(=*)\[", parse_multi_line)]
    // GMod specific
    #[regex(r"//[^\n]*", | lex | lex.slice().to_string())]
    #[regex(r"/\*(~(.*\*/.*))\*/", | lex | lex.slice().to_string())]
    #[regex(r"/\*", parse_multi_line_star)]
    Comment(String),
    #[token("...")]
    Ellipsis,
    #[token("break", | _ | Keyword::Break)]
    #[token("do", | _ | Keyword::Do)]
    #[token("else", | _ | Keyword::Else)]
    #[token("elseif", | _ | Keyword::ElseIf)]
    #[token("end", | _ | Keyword::End)]
    #[token("for", | _ | Keyword::For)]
    #[token("function", | _ | Keyword::Function)]
    #[token("if", | _ | Keyword::If)]
    #[token("in", | _ | Keyword::In)]
    #[token("local", | _ | Keyword::Local)]
    #[token("repeat", | _ | Keyword::Repeat)]
    #[token("return", | _ | Keyword::Return)]
    #[token("then", | _ | Keyword::Then)]
    #[token("until", | _ | Keyword::Until)]
    #[token("while", | _ | Keyword::While)]
    // GMod specific
    #[token("continue", | _ | Keyword::Continue)]
    Keyword(Keyword),
    #[token("{")]
    LBrace,
    #[token("[")]
    LBracket,
    #[token("false", | _ | Literal::Bool(false))]
    #[token("true", | _ | Literal::Bool(true))]
    #[token("nil", | _ | Literal::Nil)]
    #[regex(r"([0-9]+)?(\.[0-9]+)?(e(\+|-)?[0-9]+)?", | lex | {
    lex.slice().parse().map(Literal::Number).ok()
    })]
    #[regex("0x[0-9a-fA-F]+", | lex | {
    i64::from_str_radix(& lex.slice()[2..], 16)
    .map(| i | i as f64)
    .map(Literal::Number)
    .ok()
    })]
    #[regex(r#""([^"\\\n]|\\.)*""#, parse_string)]
    #[regex(r"'([^'\\\n]|\\.)*'", parse_string)]
    #[regex(r"\[(=*)\[", | lex | parse_multi_line(lex).map(Literal::String))]
    Literal(Literal),
    #[token("(")]
    LParens,
    // #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_owned())]
    #[regex(
    r"[a-zA-Z_][\x{80}-\x{31FFF}\x{E0000}-\x{E0FFF}a-zA-Z0-9_]*",
    | lex | lex.slice().to_owned()
    )]
    Name(String),
    #[token("+", | _ | Op::Add)]
    #[token("and", | _ | Op::And)]
    #[token(":", | _ | Op::Colon)]
    #[token("/", | _ | Op::Div)]
    #[token(".", | _ | Op::Dot)]
    #[token("..", | _ | Op::DotDot)]
    #[token("=", | _ | Op::Eq)]
    #[token("==", | _ | Op::EqEq)]
    #[token("^", | _ | Op::Exp)]
    #[token(">", | _ | Op::Gt)]
    #[token(">=", | _ | Op::GtEq)]
    #[token("#", | _ | Op::Len)]
    #[token("<", | _ | Op::Lt)]
    #[token("<=", | _ | Op::LtEq)]
    #[token("%", | _ | Op::Mod)]
    #[token("*", | _ | Op::Mul)]
    #[token("~=", | _ | Op::Ne)]
    #[token("not", | _ | Op::Not)]
    #[token("or", | _ | Op::Or)]
    #[token("-", | _ | Op::Sub)]
    // GMod specific
    #[token("&&", | _ | Op::And)]
    #[token("||", | _ | Op::Or)]
    #[token("!", | _ | Op::Not)]
    #[token("!=", | _ | Op::Ne)]
    Op(Op),
    #[token("}")]
    RBrace,
    #[token("]")]
    RBracket,
    #[token(")")]
    RParens,
    #[token(";")]
    Semicolon,
    #[regex(r"[\x{FEFF}]+", | lex | Some(lex.slice().chars().count()))]
    #[regex(r"[ \t\r\n\f]+", | lex | Some(lex.slice().chars().filter(| c | c == & '\n').count()))]
    Whitespace(usize),

    #[error]
    Error,
}

fn parse_string(lexer: &mut Lexer<Token>) -> Option<Literal> {
    let slice = lexer.slice();

    let pad = match slice.chars().nth(0).unwrap() {
        '[' => 2,
        _ => 1,
    };

    let mut value = String::with_capacity(slice.len());

    let mut escaped = false;

    let mut chars = slice[pad..slice.len() - pad].chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '\\' if !escaped => escaped = true,

            '0'..='9' if escaped => {
                let mut n_str = String::with_capacity(3);

                n_str.push(ch);

                for _ in 0..2 {
                    if let Some('0'..='9') = chars.peek() {
                        n_str.push(chars.next().unwrap());
                    }
                }

                let num: usize = n_str.parse().unwrap();

                if num > 255 {
                    return None;
                }

                value.push((num as u8) as char);

                escaped = false
            }

            'x' if escaped => {
                let hex_bytes = [chars.next()? as u8, chars.next()? as u8];

                let hex = ::std::str::from_utf8(&hex_bytes).ok()?;

                value.push(u8::from_str_radix(&hex, 16).ok()? as char);

                escaped = false
            }

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
        false => 0,
    };

    let len = lexer.slice().len();

    let closing = {
        let mut buf = String::with_capacity(len);

        buf.push(']');

        buf.push_str(&lexer.slice()[(offset + 1)..(len - 1)]);

        buf.push(']');

        buf
    };

    lexer
        .remainder()
        .find(&closing)
        .map(|i| lexer.bump(i + closing.len()))
        .map(|_| lexer.slice()[len..lexer.slice().len() - closing.len()].to_owned())
}

fn parse_multi_line_star(lexer: &mut Lexer<Token>) -> Option<String> {
    let len = lexer.slice().len();

    let closing = {
        let mut buf = String::with_capacity(len);

        buf.push('*');

        buf.push_str(&lexer.slice()[1..(len - 1)]);

        buf.push('/');

        buf
    };

    lexer
        .remainder()
        .find(&closing)
        .map(|i| lexer.bump(i + closing.len()))
        .map(|_| lexer.slice()[len..lexer.slice().len() - closing.len()].to_owned())
}
