use std::hint::black_box;

use bumpalo::Bump;
use glua::Parser;

fn main() {
    for _ in 0..8192 {
        let bump = Bump::new();

        let tokens = Parser::lex(include_str!("../test.lua"), &bump).unwrap();

        let mut parser = Parser::new_in(&tokens, &bump);

        black_box(parser.parse_chunk().unwrap());
    }
}
