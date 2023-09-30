use glua::lexer::lex;
use glua::Parser;
use std::hint::black_box;

fn main() {
    for _ in 0..256 {
        let tokens = lex(include_str!("../test.lua")).unwrap();

        let mut parser = Parser::new(tokens);

        black_box(parser.parse_chunk().unwrap());
    }
}
