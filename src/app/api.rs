//! Public application API facade.

use crate::adapters::FilesystemItemStore;
use crate::app::{AppContext, commands};
use crate::domain::AppError;

/// Create the default application context.
fn default_context() -> Result<AppContext<FilesystemItemStore>, AppError> {
    let store = FilesystemItemStore::from_env()?;
    Ok(AppContext::new(store))
}

/// Add a new item to storage using the default backend.
pub fn add(id: &str, content: &str) -> Result<(), AppError> {
    let ctx = default_context()?;
    commands::add::execute(&ctx, id, content)
}

/// List all stored item identifiers.
pub fn list() -> Result<Vec<String>, AppError> {
    let ctx = default_context()?;
    commands::list::execute(&ctx)
}

/// Delete an item from storage using the default backend.
pub fn delete(id: &str) -> Result<(), AppError> {
    let ctx = default_context()?;
    commands::delete::execute(&ctx, id)
}
