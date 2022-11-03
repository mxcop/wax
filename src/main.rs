use clap::Parser;
use args::Commands;

mod args;
mod create;
mod build;
mod flow;
mod utils;

fn main() {

  let args = args::Args::parse();

  match args.cmd {
    Commands::Create { name } => create::create(name),
    Commands::Build { path } => build::build(path),
  }
}