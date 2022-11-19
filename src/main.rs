use clap::Parser;
use args::Commands;

mod args;
mod create;
mod build;
mod parser;
mod utils;
mod server;
mod fetcher;
mod error;

#[tokio::main]
async fn main() {

  // Enable colors in the command prompt.
  colored::control::set_virtual_terminal(true).unwrap();

  let args = args::Args::parse();

  match args.cmd {
    Commands::Create { name } => create::create(name),
    Commands::Build { path } => { build::build(path.clone()); /* server::start(8080, format!("{}/dist/", &path), false, "", "127.0.0.1").await; */ },
  }
}