use std::time::Duration;

use bumpalo::Bump;
use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use glua::{lexer::Token, Parser};
use logos::Logos;

static CODE: &'static str = include_str!("../test.lua");

fn lexer(c: &mut Criterion) {
    c.benchmark_group("lexer")
        .sample_size(10_000)
        .warm_up_time(Duration::from_secs(10))
        .measurement_time(Duration::from_secs(30))
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
        .sample_size(10_000)
        .warm_up_time(Duration::from_secs(10))
        .measurement_time(Duration::from_secs(30))
        .bench_function("lex", |b| {
            b.iter(|| {
                let bump = Bump::new();

                let _ = black_box(Parser::lex(CODE, &bump));
            })
        })
        .bench_function("parse_chunk", |b| {
            let lexer_bump = Bump::new();
            let tokens = Parser::lex(CODE, &lexer_bump).unwrap();

            b.iter(|| {
                let bump = Bump::new();

                let mut parser = Parser::new_in(&tokens, &bump);

                let _ = black_box(parser.parse_chunk());
            })
        })
        .bench_function("parse_full", |b| {
            b.iter(|| {
                let bump = Bump::new();

                let tokens = Parser::lex(CODE, &bump).unwrap();
                let mut parser = Parser::new_in(&tokens, &bump);

                let _ = black_box(parser.parse_chunk());
            })
        });
}

criterion_group!(benches, lexer, parser);
criterion_main!(benches);
