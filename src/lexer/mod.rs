mod keyword;
mod literal;
mod op;
mod token;

pub use keyword::Keyword;
pub use literal::Literal;
pub use op::Op;
pub use token::Token;

pub struct Lexer;


pub fn lex(mut input: &str) -> Result<Vec<Token>, String> {
    mod lexer_impl {
        use super::{
            Keyword,
            Literal,
            Op,
            Token,
        };
        use plex::lexer;

        lexer! {
            pub fn take_token(text: 'a) -> Result<Token, String>;
            // Whitespace
            r"[ \t\r\n]" => Ok(Token::Whitespace(text.chars().filter(|c| c == &'\n').count())),


            // Comment
            r"//[^\n]*"                   => Ok(Token::Comment(text.to_owned())),
            r"--[^\n]*"                   => Ok(Token::Comment(text.to_owned())),
            r"/\*(~(.*\*/.*))\*/"           => Ok(Token::Comment(text.to_owned())),
            r"--\[\[(~(.*--\]\]/.*))--\]\]" => Ok(Token::Comment(text.to_owned())),


            // Generic
            r"\{" => Ok(Token::LBrace),
            r"\}" => Ok(Token::RBrace),
            r"\(" => Ok(Token::LParens),
            r"\)" => Ok(Token::RParens),
            r"\[" => Ok(Token::LBracket),
            r"\]" => Ok(Token::RBracket),
            ","   => Ok(Token::Comma),
            ";"   => Ok(Token::Semicolon),
            r"\.\.\." => Ok(Token::Ellipsis),


            // Keyword
            "and"      => Ok(Op::And.into()),
            "break"    => Ok(Keyword::Break.into()),
            "do"       => Ok(Keyword::Do.into()),
            "else"     => Ok(Keyword::Else.into()),
            "elseif"   => Ok(Keyword::ElseIf.into()),
            "end"      => Ok(Keyword::End.into()),
            "for"      => Ok(Keyword::For.into()),
            "function" => Ok(Keyword::Function.into()),
            "if"       => Ok(Keyword::If.into()),
            "in"       => Ok(Keyword::In.into()),
            "local"    => Ok(Keyword::Local.into()),
            "not"      => Ok(Op::Not.into()),
            "or"       => Ok(Op::Or.into()),
            "repeat"   => Ok(Keyword::Repeat.into()),
            "return"   => Ok(Keyword::Return.into()),
            "then"     => Ok(Keyword::Then.into()),
            "until"    => Ok(Keyword::Until.into()),
            "while"    => Ok(Keyword::While.into()),
            "continue" => Ok(Keyword::Continue.into()),


            // Literals
                // Boolean
                "true"  => Ok(Literal::Bool(true).into()),
                "false" => Ok(Literal::Bool(false).into()),

                // Nil
                "nil" => Ok(Literal::Nil.into()),

                // Float
                r"[0-9]+\.[0-9]+" => {
                    if let Ok(f) = text.parse::<f64>() {
                        Ok(Literal::Number(f).into())
                    } else {
                        Err(format!("Float {:?} is out of range", text))
                    }
                }

                // Integer
                "[0-9]+" => {
                    if let Ok(i) = text.parse::<f64>() {
                        Ok(Literal::Number(i).into())
                    } else {
                        Err(format!("Integer {:?} is out of range", text))
                    }
                }

                // String
                r#""(\\.|[^"\\])*""# => Ok(Literal::String(text[1..text.len() - 1].to_owned()).into()),


            // Operators
                r"\+"   => Ok(Op::Add.into()),
                ":"     => Ok(Op::Colon.into()),
                "/"     => Ok(Op::Div.into()),
                r"\."   => Ok(Op::Dot.into()),
                r"\.\." => Ok(Op::DotDot.into()),
                "="     => Ok(Op::Eq.into()),
                "=="    => Ok(Op::EqEq.into()),
                r"\^"   => Ok(Op::Exp.into()),
                ">"     => Ok(Op::Gt.into()),
                ">="    => Ok(Op::GtEq.into()),
                "#"     => Ok(Op::Len.into()),
                "<"     => Ok(Op::Lt.into()),
                "<="    => Ok(Op::LtEq.into()),
                "%"     => Ok(Op::Mod.into()),
                r"\*"   => Ok(Op::Mul.into()),
                "!="    => Ok(Op::Ne.into()),
                "-"     => Ok(Op::Sub.into()),

                // GMod specific
                r"\&\&" => Ok(Op::And.into()),
                r"\|\|" => Ok(Op::Or.into()),
                "!"     => Ok(Op::Not.into()),
                "!="    => Ok(Op::Ne.into()),

            // Identifier
            "[a-zA-Z_][a-zA-Z0-9_]*" => Ok(Token::Name(text.to_owned())),

            // Bad character
            "." => Err(format!("Unexpected character {:?} in input", text))
        }
    }

    let mut tokens = Vec::new();

    while let Some((token, remaining)) = lexer_impl::take_token(input) {
        input = remaining;

        tokens.push(token?);
    }

    Ok(tokens)
}
