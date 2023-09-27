/*#![feature(test)]
extern crate test;*/

pub use self::parser::Parser;

pub mod ast;
pub mod lexer;
pub mod parser;

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;
    // use test::bench::Bencher;

    use crate::lexer::lex;
    use crate::Parser;

    static CODE: &'static str = include_str!("../test.lua");

    #[test]
    fn it_works() {
        let tokens = lex(CODE).unwrap();

        // dbg!(&tokens);

        let mut parser = Parser::new(tokens);

        let chunk = parser.parse_chunk().unwrap();

        // dbg!(&chunk);

        write!(File::create("test.parsed").unwrap(), "{:#?}", chunk).unwrap();
    }

    /*    #[bench]
    fn lexer(b: &mut Bencher) {
        b.iter(|| lex(CODE).unwrap());
    }

    #[bench]
    fn parser(b: &mut Bencher) {
        b.iter(|| {
            let tokens = lex(CODE).unwrap();

            let mut parser = Parser::new(tokens);

            parser.parse_chunk().unwrap()
        });
    }*/
}
