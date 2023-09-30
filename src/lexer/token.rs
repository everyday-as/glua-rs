use bumpalo::{
    collections::{String as BumpString, Vec},
    Bump,
};
use logos::{Lexer, Logos, Source};
use memchr::memmem;

use crate::lexer::{Keyword, Literal, Op};

#[derive(Clone, Copy, Debug, Logos, PartialEq)]
#[logos(skip r"[ \t\r\n\f\x{FEFF}]+", extras = & 's Bump)]
pub enum Token<'a> {
    #[token(",")]
    Comma,
    #[token("//", comment)]
    #[token("/*", comment)]
    #[token("--", comment)]
    #[token("--[", comment)]
    Comment(&'a str),
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
    #[token("goto", | _ | Keyword::Goto)]
    Keyword(Keyword),
    #[token("{")]
    LBrace,
    #[token("[")]
    LBracket,
    #[token("false", | _ | Literal::Bool(false))]
    #[token("true", | _ | Literal::Bool(true))]
    #[token("nil", | _ | Literal::Nil)]
    #[regex(r"([0-9]+)?(\.)?([0-9]+)(\.)?(e(\+|-)?[0-9]+)?", | lex | {
    lex.slice().parse().map(Literal::Number).ok()
    })]
    #[regex("0x[0-9a-fA-F]+", | lex | {
    i64::from_str_radix(& lex.slice()[2..], 16)
    .map(| i | i as f64)
    .map(Literal::Number)
    .ok()
    })]
    #[regex(r#""([^"\\\n]|\\.)*""#, | lex | string_literal(lex).map(Literal::String))]
    #[regex(r"'([^'\\\n]|\\.)*'", | lex | string_literal(lex).map(Literal::String))]
    #[regex(r"\[(=*)\[", | lex | multi_line(lex).map(Literal::String))]
    Literal(Literal<'a>),
    #[token("(")]
    LParens,
    // #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_owned())]
    #[regex(
    r"[a-zA-Z_\x{0400}-\x{04FF}][\x{80}-\x{31FFF}\x{E0000}-\x{E0FFF}\x{0400}-\x{04FF}a-zA-Z0-9_]*",
    | lex | lex.slice()
    )]
    Name(&'a str),
    #[regex(r"::[a-zA-Z_0-9]+::", | lex | & lex.slice()[2..lex.slice().len() - 2])]
    Label(&'a str),
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
}

impl From<Keyword> for Token<'_> {
    fn from(value: Keyword) -> Self {
        Self::Keyword(value)
    }
}

impl<'a> From<Literal<'a>> for Token<'a> {
    fn from(value: Literal<'a>) -> Self {
        Self::Literal(value)
    }
}

impl From<Op> for Token<'_> {
    fn from(value: Op) -> Self {
        Self::Op(value)
    }
}

fn string_literal<'a>(lexer: &Lexer<'a, Token<'a>>) -> Option<&'a str> {
    let slice = lexer.slice().as_bytes();

    let pad = slice.starts_with(b"[") as usize + 1;

    let mut value = Vec::new_in(lexer.extras);

    let mut base = pad;
    for offset in memchr::memchr_iter(b'\\', slice) {
        if offset <= base {
            continue;
        }

        value.extend_from_slice(&slice[base..offset]);

        match slice[offset + 1] {
            b'a' => {
                value.push(7);

                base = offset + 2;
            }

            b'b' => {
                value.push(8);

                base = offset + 2;
            }

            b'f' => {
                value.push(12);

                base = offset + 2;
            }

            b'n' => {
                value.push(b'\n');

                base = offset + 2;
            }

            b'r' => {
                value.push(b'\r');

                base = offset + 2;
            }

            b't' => {
                value.push(b'\t');

                base = offset + 2;
            }

            b'v' => {
                value.push(11);

                base = offset + 2;
            }

            b'\\' | b'"' | b'\'' |  b'\n' => {
                value.push(slice[offset + 1]);

                base = offset + 2;
            }


            b'0'..=b'9' => {
                let mut end = offset + 2;

                for _ in 0..2 {
                    if let Some(b'0'..=b'9') = slice.get(end) {
                        end += 1;
                    }
                }

                let n: u16 = std::str::from_utf8(&slice[offset + 1..end])
                    .ok()?
                    .parse()
                    .unwrap();

                if n > 255 {
                    return None;
                }

                value.push(n as u8);

                base = end
            }

            b'x' => {
                let hex = std::str::from_utf8(slice.slice(offset + 1..offset + 3)?).ok()?;

                value.push(u8::from_str_radix(hex, 16).ok()?);

                base = offset + 3;
            }

            _ => return None,
        }
    }

    if base == pad {
        return Some(&lexer.slice()[pad..slice.len() - pad]);
    } else if base < slice.len() - pad {
        value.extend_from_slice(&slice[base..slice.len() - pad])
    }

    let string = BumpString::from_utf8(value).ok()?;

    Some(string.into_bump_str())
}

fn comment<'a>(lexer: &mut Lexer<'a, Token<'a>>) -> Option<&'a str> {
    // Multi-line comment
    // `slice` may be "--[": https://github.com/maciejhirsz/logos/issues/315#issuecomment-1714257180
    // We cannot conditionally match [=*[ because it generates false matches, so we match the prefix
    // --[ which may or may not be a multi-line comment. We will first attempt match a multi-line
    // comment, or fallthrough if it's a single line comment that happens to start with "["
    if lexer.slice().len() == 3 && lexer.remainder().starts_with(['=', '[']) {
        let offset = memchr::memchr(b'[', lexer.remainder().as_bytes())?;

        lexer.bump(offset + 1);

        return multi_line(lexer);
    }

    // C-Style multi-line comment
    if lexer.slice() == "/*" {
        return match memmem::find(lexer.remainder().as_bytes(), b"*/") {
            None => {
                lexer.bump(lexer.remainder().len());

                Some(&lexer.slice()[2..])
            }
            Some(end) => {
                lexer.bump(end + 2);

                Some(&lexer.slice()[2..end + 4])
            }
        };
    }

    let remainder = lexer.remainder();
    return match memchr::memchr(b'\n', remainder.as_bytes()) {
        None => {
            lexer.bump(remainder.len());

            Some(remainder)
        }

        Some(offset) => {
            lexer.bump(offset);

            // Note that using 2 as an offset is valid even for "--[" because the "[" is part of
            // the comment in this branch.
            Some(&lexer.slice()[2..])
        }
    };
}

fn multi_line<'a>(lexer: &mut Lexer<'a, Token<'a>>) -> Option<&'a str> {
    let slice = lexer.slice();

    // Ideally we could create a sub-lexer without this prefix in `comment`
    let offset = 2 * slice.starts_with('-') as usize;

    let len = slice.len();

    let closing = {
        let mut buf = String::with_capacity(len - offset);

        buf.push(']');

        buf.push_str(&lexer.slice()[(offset + 1)..(len - 1)]);

        buf.push(']');

        buf
    };

    memmem::find(lexer.remainder().as_bytes(), closing.as_bytes())
        .map(|i| lexer.bump(i + closing.len()))
        .map(|_| &lexer.slice()[len..lexer.slice().len() - closing.len()])
}
