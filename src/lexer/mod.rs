use logos::{Logos, Span};

pub use keyword::Keyword;
pub use literal::Literal;
pub use op::Op;
pub use token::Token;

mod keyword;
mod literal;
mod op;
mod token;

pub fn lex(input: &str) -> Result<Vec<(Token, Span)>, String> {
    let mut lexer = Token::lexer(input).spanned();

    let mut tokens = Vec::new();

    while let Some((token, span)) = lexer.next() {
        tokens.push((
            match token {
                Token::Error => Err(format!("Unexpected token `{:?}` in input", span)),
                _ => Ok(token),
            }?,
            span,
        ));
    }

    Ok(tokens)
}
