use super::item_id::ItemId;
use crate::AppError;

/// Storage contract for item operations.
pub trait ItemStore {
    /// Store an item with the given identifier and content.
    fn add_item(&self, id: &ItemId, content: &str) -> Result<(), AppError>;

    /// List all stored item identifiers.
    fn list_items(&self) -> Result<Vec<String>, AppError>;

    /// Remove an item by identifier.
    fn delete_item(&self, id: &ItemId) -> Result<(), AppError>;
}
