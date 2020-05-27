use logos::Logos;

pub use keyword::Keyword;
pub use literal::Literal;
pub use op::Op;
pub use token::Token;

mod keyword;
mod literal;
mod op;
mod token;

pub fn lex(input: &str) -> Result<Vec<Token>, String> {
    let mut lexer = Token::lexer(input);

    let mut tokens = Vec::new();

    while let Some(token) = lexer.next() {
        tokens.push(match token {
            Token::Error => Err(format!("Unexpected token `{:?}` in input", lexer.slice())),
            _ => Ok(token)
        }?);
    }

    Ok(tokens)
}
