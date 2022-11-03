use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "Wax")]
#[command(author = "Max C. <mxcop.dev@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "A Simple HTML Framework", long_about = None)]
pub struct Args {
  /// Name of the person to greet
  #[command(subcommand)]
  pub cmd: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Create {
        name: String,
    },
    Build {
        path: String,
    },
}