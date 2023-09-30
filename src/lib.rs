// #![feature(test)]

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

    use crate::Parser;

    static CODE: &'static str = include_str!("../test.lua");

    #[test]
    fn it_works() {
        let bump = Bump::new();

        let mut parser = Parser::try_from_str_in(CODE, &bump).unwrap();

        let chunk = parser.parse_chunk()
            .map_err(|e| e.to_string())
            .unwrap();

        println!("Allocated: {}", convert(bump.allocated_bytes() as f64));
        // println!("Wasted: {}", convert(parser.waste as f64));

        let file = File::create("test.parsed").unwrap();

        let mut writer = BufWriter::new(file);

        write!(writer, "{:#?}", chunk).unwrap();
    }
}

/*#[cfg(test)]
mod benches {
    extern crate test;

    use std::hint::black_box;

    use bumpalo::Bump;
    use logos::Logos;
    use test::bench::Bencher;

    use crate::{lexer::Token, Parser};

    static CODE: &'static str = include_str!("../test.lua");

    #[bench]
    fn lexer(b: &mut Bencher) {
        b.iter(|| {
            let bump = Bump::new();

            Token::lexer_with_extras(CODE, &bump)
                .map(black_box)
                .for_each(drop)
        });
    }

    #[bench]
    fn parser(b: &mut Bencher) {
        b.iter(|| {
            let bump = Bump::new();

            let mut parser = Parser::try_from_str_in(CODE, &bump).unwrap();

            black_box(parser.parse_chunk().unwrap());
        });
    }
}
*/