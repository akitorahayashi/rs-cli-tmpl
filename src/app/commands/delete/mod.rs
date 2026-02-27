use crate::app::AppContext;
use crate::domain::ports::ItemStore;
use crate::domain::{AppError, ItemId};

/// Delete an item from storage.
pub fn execute(ctx: &AppContext<impl ItemStore>, id: &str) -> Result<(), AppError> {
    let id = ItemId::new(id)?;
    ctx.store().delete_item(&id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::MockItemStore;

    #[test]
    fn delete_item_forwards_to_store() {
        let store = MockItemStore::default();
        let ctx = AppContext::new(store);

        execute(&ctx, "demo").expect("execution should succeed");

        let calls = ctx.store().delete_calls.borrow();
        assert_eq!(calls.as_slice(), ["demo".to_string()]);
    }

    #[test]
    fn delete_item_rejects_invalid_id() {
        let store = MockItemStore::default();
        let ctx = AppContext::new(store);

        let result = execute(&ctx, "invalid/id");
        assert!(result.is_err());
    }
}
