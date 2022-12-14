use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let input = &std::fs::read_to_string("./example/src/pages/hive.wx").expect("failed to load file");

    c.bench_function("lexing + parsing", |b| b.iter(|| {
        // Initialize the lexical iterator:
        let lexer = black_box(waxc_lexer::lex(input));
        let iter = black_box(waxc_lexer::lexer::LexIter::new(lexer));

        // Start the parsing process:
        black_box(waxc_parser::parse(input.to_string(), iter).unwrap());
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);