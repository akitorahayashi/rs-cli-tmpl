//! CLI adapter.

mod add;
mod delete;
mod list;

use crate::AppError;
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
    #[command(about = "Add a new item", visible_alias = "a")]
    Add {
        id: String,
        #[arg(short, long)]
        content: String,
    },
    #[command(about = "List all items", visible_alias = "ls")]
    List,
    #[command(about = "Delete an item", visible_alias = "rm")]
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
