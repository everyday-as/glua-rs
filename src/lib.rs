extern crate core;

pub use self::lexer::lex;
pub use self::parser::Parser;

pub mod ast;
pub mod lexer;
pub mod parser;


#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{Read, Write};

    use crate::lexer::lex;
    use crate::Parser;

    #[test]
    fn it_works() {
        let lua = {
            let mut file = File::open("test.lua").unwrap();

            let mut buf = String::new();

            file.read_to_string(&mut buf);

            buf
        };

        let mut parser = Parser::new(lex(&lua).unwrap());

        let chunk = parser.parse_chunk().unwrap();

        write!(File::create("test.parsed").unwrap(), "{:#?}", chunk);
    }
}

