pub use self::parser::Parser;

pub mod ast;
pub mod lexer;
pub mod parser;

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufWriter, Write},
    };

    use bumpalo::Bump;

    use crate::{Parser, parser::Error};

    static CODE: &'static str = include_str!("../test.lua");

    #[test]
    fn it_works() {
        let bump = Bump::new();

        let tokens = unwrap(Parser::lex(CODE, &bump));

        println!("Token count: {}", tokens.len());

        let mut parser = Parser::new_in(&tokens, &bump);

        let chunk = unwrap(parser.parse_chunk());

        #[cfg(debug_assertions)]
        {
            println!(
                "Allocated: {} ({} wasted)",
                pretty_bytes::converter::convert(bump.allocated_bytes() as f64),
                pretty_bytes::converter::convert(parser.stats.wasted_mem as f64)
            );
            println!(
                "Rewinds: {} ({} tokens, {}% overhead)",
                parser.stats.rewinds,
                parser.stats.rewind_tok,
                ((parser.stats.rewind_tok as f64 / tokens.len() as f64) * 100. * 100.).round()
                    / 100.
            );
        }

        let ast_file = File::create("test.ast").unwrap();
        let str_file = File::create("test.out.lua").unwrap();

        write!(BufWriter::new(ast_file), "{:#?}", chunk).unwrap();
        write!(BufWriter::new(str_file), "{}", chunk).unwrap();
    }

    fn unwrap<T>(res: Result<T, Error>) -> T {
        match res {
            Err(err) => match err {
                Error::Lexer(ref span) => {
                    panic!("{:?} in `{}`", err, &CODE[span.start..span.end]);
                }
                _ => panic!("{:?}", err),
            },

            Ok(value) => value,
        }
    }
}
