mod ast;
mod parser;

mod lexer;

#[cfg(test)]
mod tests {
    use crate::lexer::lex;
    use std::fs::File;
    use std::io::Read;
    use crate::parser::Parser;

    #[test]
    fn it_works() {
        let lua = {
            let mut file = File::open("test.lua").unwrap();

            let mut buf = String::new();

            file.read_to_string(&mut buf);

            buf
        };

        let mut parser = Parser::new(lex(&lua).unwrap());

        let mut exp = parser.parse_chunk().unwrap();

        println!("{:#?}", exp);
        // println!("{:#?}", exp.eval())
    }
}
