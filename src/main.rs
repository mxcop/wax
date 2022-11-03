use clap::Parser;
use args::Commands;
use colored::Colorize;

mod args;
mod build;
mod flow;
mod utils;

fn main() {

  let args = args::Args::parse();

  match args.cmd {
    Commands::Create { name } => println!("\n{} creating project '{}'", "Wax".green().bold(), &name),
    Commands::Build { path } => build::build(path),
  }
}