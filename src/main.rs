use std::path::Path;

//use waxc_lexer::lexer::LexIter;

mod args;
mod build;
mod create;
mod utils;

fn main() {
  // Enable colors in the command prompt.
  //colored::control::set_virtual_terminal(true).unwrap();

  //let index_file = std::fs::read_to_string("./example/src/index.html").expect("failed to load index.html");
  //let input = std::fs::read_to_string("./example/src/pages/hive.wx").expect("failed to load file");

  run(Path::new("./example/src/index.html"), "src/pages/hive.wx");
}

/// Run a single parsing.
fn run(index_path: &Path, filename: &str) {
  // Initialize the lexical iterator:
  //let lexer = waxc_lexer::lex(input);
  //let iter = LexIter::new(lexer);

  // Start the parsing process:
  //let parser = waxc_parser::parse(input.to_string(), iter);

  // Check for errors:
  // let ast = match parser {
  //   Err(e) => {
  //     e.print(input, filename);
  //     return;
  //   }
  //   Ok(ast) => ast
  // };
  // println!("{}", &ast);

  // Generate the pages:
  let pages = waxc_codegen::generate(index_path, Path::new("./example/src/pages/"));

  // Check for errors:
  let pages = match pages {
    Err(e) => {
      e.print("./example/src/");
      return;
    }
    Ok(pages) => pages
  };
  
  // for page in pages {
  //   println!("{}", &page);
  // }

  // Save the files:
  if std::path::Path::new("./build/").is_dir() == false {
    std::fs::create_dir("./build/").expect("failed to create build dir");
  }

  for page in pages {
    let Ok(page_path) = Path::new(&page.page_path).strip_prefix("./example/src/pages") else {
      panic!("PANIC!");
    };
    let Some(page_name) = page_path.file_stem() else {
      panic!("PANIC!");
    };
    //dbg!(page_path.parent().unwrap(), page_name);
    let page_name = page_name.to_string_lossy().to_string();

    let output_path = Path::new("./build").join(page_path.parent().unwrap()).join(format!("{}.html", page_name));
    let output_path2 = output_path.as_path();
    dbg!(output_path2);
    if output_path2.exists() == false {
      std::fs::create_dir_all(output_path2.parent().unwrap()).expect("failed to create page directory");
    }
    std::fs::write(output_path2, page.html).expect("failed to save html");
    //std::fs::write(format!("./build/{}", &page_path), comb.css).expect("failed to save css");
  }
}
