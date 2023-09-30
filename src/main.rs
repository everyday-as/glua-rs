use std::hint::black_box;

use bumpalo::Bump;
use glua::Parser;

fn main() {
    for _ in 0..8192 {
        let bump = Bump::new();

        let mut parser = Parser::try_from_str_in(include_str!("../test2.lua"), &bump).unwrap();

        black_box(parser.parse_chunk().unwrap());
    }
}
