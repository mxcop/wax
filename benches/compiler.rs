use std::path::Path;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
  let index = &std::fs::read_to_string("./example/src/index.html").expect("failed to load file");
  let input = &std::fs::read_to_string("./example/src/pages/index.wx").expect("failed to load file");

  c.bench_function("lexing", |b| {
    b.iter(|| {
      // Initialize the lexical iterator:
      let lexer = black_box(waxc_lexer::lex(input));
      let mut iter = black_box(waxc_lexer::lexer::LexIter::new(lexer));

      black_box(while let Some(_) = iter.next() {});
    })
  });

  c.bench_function("lexing + parsing", |b| {
    b.iter(|| {
      // Initialize the lexical iterator:
      let lexer = black_box(waxc_lexer::lex(input));
      let iter = black_box(waxc_lexer::lexer::LexIter::new(lexer));

      // Start the parsing process:
      black_box(waxc_parser::parse(input.to_string(), Path::new("./index.html"), iter).unwrap());
    })
  });

  c.bench_function("lexing + parsing + codegen", |b| {
    b.iter(|| {
      // Initialize the lexical iterator:
      //let lexer = black_box(waxc_lexer::lex(input));
      //let iter = black_box(waxc_lexer::lexer::LexIter::new(lexer));

      // Start the parsing process:
      //let ast = black_box(waxc_parser::parse(input.to_string(), Path::new("./index.html"), iter).unwrap());

      // Start the codegen process:
      black_box(waxc_codegen::generate(Path::new("./example/src/index.html"), Path::new("./example/src/pages/")).unwrap());
    })
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
