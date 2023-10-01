use std::time::Duration;

use bumpalo::Bump;
use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use glua::{lexer::Token, Parser};
use logos::Logos;

static CODE: &'static str = include_str!("../test.lua");

fn lexer(c: &mut Criterion) {
    c.benchmark_group("lexer")
        .sample_size(1_000)
        .significance_level(0.01)
        .throughput(Throughput::Bytes(CODE.len() as u64))
        .bench_function("raw", |b| {
            b.iter(|| {
                let bump = Bump::new();

                Token::lexer_with_extras(CODE, &bump).for_each(|_| ());
            })
        });
}

fn parser(c: &mut Criterion) {
    c.benchmark_group("parser")
        .sample_size(3000)
        .measurement_time(Duration::from_secs(30))
        .significance_level(0.01)
        .bench_function("lex", |b| {
            let mut bump = Bump::new();

            b.iter(|| {
                let _ = Parser::lex(CODE, &bump);

                bump.reset();
            })
        })
        .bench_function("parse_chunk", |b| {
            let lexer_bump = Bump::new();
            let tokens = Parser::lex(CODE, &lexer_bump).unwrap();

            let mut parser_bump = Bump::new();
            b.iter(|| {
                let mut parser = Parser::new_in(&tokens, &parser_bump);
                let _ = black_box(parser.parse_chunk());
                parser_bump.reset();
            });
        })
        .bench_function("parse_full", |b| {
            let mut bump = Bump::new();

            b.iter(|| {
                let tokens = Parser::lex(CODE, &bump).unwrap();
                let mut parser = Parser::new_in(&tokens, &bump);

                let _ = black_box(parser.parse_chunk());

                bump.reset();
            })
        });
}

criterion_group!(benches, lexer, parser);
criterion_main!(benches);
