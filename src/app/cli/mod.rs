//! CLI adapter.

mod add;
mod delete;
mod list;

use crate::domain::AppError;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rs-cli-tmpl")]
#[command(version)]
#[command(
    about = "Reference architecture for building Rust CLI tools",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(visible_alias = "a")]
    Add {
        id: String,
        #[clap(short, long)]
        content: String,
    },
    #[clap(visible_alias = "ls")]
    List,
    #[clap(visible_alias = "rm")]
    Delete { id: String },
}

/// Entry point for the CLI.
pub fn run() {
    let cli = Cli::parse();

    let result: Result<(), AppError> = match cli.command {
        Commands::Add { id, content } => add::run(&id, &content),
        Commands::List => list::run(),
        Commands::Delete { id } => delete::run(&id),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
