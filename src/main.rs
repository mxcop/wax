use clap::Parser;
use args::Commands;

mod args;
mod create;
mod build;
mod utils;
mod logging;

fn main() {

  // Enable colors in the command prompt.
  colored::control::set_virtual_terminal(true).unwrap();

  let args = args::Args::parse();

  match args.cmd {
    Commands::Create { name } => create::create(name),
    Commands::Build { path } => build::build(path.clone()),
  }
}