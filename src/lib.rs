//! Library entry point exposing the core command handlers.

pub mod app;
pub mod adapters;
pub mod domain;
pub mod ports;

#[cfg(test)]
pub(crate) mod testing;

use app::{
    AppContext, Command,
    commands::{AddItem, DeleteItem, ListItems},
};
use adapters::FilesystemItemStore;

pub use domain::AppError;

/// Create the default application context.
fn default_context() -> Result<AppContext<FilesystemItemStore>, AppError> {
    let store = FilesystemItemStore::from_env()?;
    Ok(AppContext::new(store))
}

/// Add a new item to storage using the default filesystem backend.
pub fn add(id: &str, content: &str) -> Result<(), AppError> {
    let ctx = default_context()?;
    let command = AddItem { id, content };

    command.execute(&ctx)?;
    println!("✅ Added item '{id}'");
    Ok(())
}

/// List all stored item identifiers.
pub fn list() -> Result<Vec<String>, AppError> {
    let ctx = default_context()?;
    let command = ListItems;
    let items = command.execute(&ctx)?;

    println!("📦 Stored items:");
    if items.is_empty() {
        println!("(none)");
    } else {
        for id in &items {
            println!("- {id}");
        }
    }

    Ok(items)
}

/// Delete an item from storage.
pub fn delete(id: &str) -> Result<(), AppError> {
    let ctx = default_context()?;
    let command = DeleteItem { id };

    command.execute(&ctx)?;
    println!("🗑️  Deleted item '{id}'");
    Ok(())
}
