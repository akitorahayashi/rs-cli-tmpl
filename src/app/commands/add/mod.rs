use crate::app::AppContext;
use crate::domain::ports::ItemStore;
use crate::domain::{AppError, ItemId};

/// Add an item to storage.
pub fn execute(ctx: &AppContext<impl ItemStore>, id: &str, content: &str) -> Result<(), AppError> {
    let id = ItemId::new(id)?;
    ctx.store().add_item(&id, content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::MockItemStore;

    #[test]
    fn add_item_forwards_to_store() {
        let store = MockItemStore::default();
        let ctx = AppContext::new(store);

        execute(&ctx, "demo", "example").expect("execution should succeed");

        let calls = ctx.store().add_calls.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0], ("demo".to_string(), "example".to_string()));
    }

    #[test]
    fn add_item_rejects_invalid_id() {
        let store = MockItemStore::default();
        let ctx = AppContext::new(store);

        let result = execute(&ctx, "invalid/id", "example");
        assert!(result.is_err());
    }
}
