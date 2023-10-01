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
    use pretty_bytes::converter::convert;

    use crate::{parser::Error, Parser};

    static CODE: &'static str = include_str!("../test.lua");

    #[test]
    fn it_works() {
        let bump = Bump::new();

        let tokens = unwrap(Parser::lex(CODE, &bump));

        let mut parser = Parser::new_in(&tokens, &bump);

        let chunk = unwrap(parser.parse_chunk());

        println!("Allocated: {}", convert(bump.allocated_bytes() as f64));

        let file = File::create("test.parsed").unwrap();

        let mut writer = BufWriter::new(file);

        write!(writer, "{:#?}", chunk).unwrap();
        // println!("Wasted: {}", convert(parser.waste as f64));
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
