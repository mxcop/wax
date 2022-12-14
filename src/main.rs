use waxc_lexer::lexer::LexIter;

mod args;
mod build;
mod create;
mod utils;

fn main() {
  // Enable colors in the command prompt.
  colored::control::set_virtual_terminal(true).unwrap();

  let input = std::fs::read_to_string("./example/src/pages/hive.wx").expect("failed to load file");

  run(&input, "src/pages/hive.wx");
}

/// Run a single parsing.
fn run(input: &str, _filename: &str) {
  let start = std::time::Instant::now();

  // Initialize the lexical iterator:
  let lexer = waxc_lexer::lex(input);
  let iter = LexIter::new(lexer);

  let lex_time = start.elapsed().as_nanos();
  let start = std::time::Instant::now();

  // Start the parsing process:
  let parser = waxc_parser::parse(input.to_string(), iter).unwrap();

  let time = start.elapsed().as_nanos();

  println!("{}", parser);

  println!("\nLexing time : {}s ({}ms) ({}µs) ({}ns)", lex_time as f32 / 1_000_000_000f32, lex_time as f32 / 1_000_000f32, lex_time as f32 / 1000f32, lex_time);
  println!("\nParsing time : {}s ({}ms) ({}µs) ({}ns)", time as f32 / 1_000_000_000f32, time as f32 / 1_000_000f32, time as f32 / 1000f32, time);

  // Run some tests:
  //let mut tokens: Vec<Token> = Vec::with_capacity(256);
  //let mut pos = 0usize;

  // while let Some(tk) = lexer.next() {
  //   tokens.push(tk);
  // }

  // let lex_time = start.elapsed().as_nanos();

  // // Debug output:
  // for tk in &tokens {
  //   let text = input.get(pos..pos+tk.get_len());
  //   if let Some(text) = text {
  //     println!("{:?} : {}", text, format!("{:?}", tk).bright_black());
  //   }
  //   pos += tk.get_len();
  // }

  //println!("\nLexing time : {}s ({}µs) ({}ns)", lex_time as f32 / 1_000_000_000f32, lex_time as f32 / 1000f32, lex_time);
  //println!("Token total : {}", tokens.len());
}

const INPUT: &'static str = r#"/* Html Template */
tmpl card:
  <div class="card">
    <h2 #header title='let " const'>Test</h2>
    <p #desc>tmpl</p>
    <br>
    <img src="img.png">
  </div>;

/* Template Implementation */
impl card(header: str, desc: str) {
  #header.inner_text = header;
  #desc.inner_text = desc;
}

/* Css Stylesheet */
styl card() {
  .card {
    width: 300px;
    height: 400px;
  }
}

/* Root Html Template */
tmpl @html:
  <body>
    <-card header="Card 1" desc="This is card one" />
    <-card header="Card 2" desc="This is card two">
      <p>Hello world</p>
    <-/card>
  </body>;"#;