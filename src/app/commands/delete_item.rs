use crate::app::{AppContext, Command};
use crate::domain::{AppError, ItemId};
use crate::ports::ItemStore;

/// Command to delete an item from storage.
pub struct DeleteItem<'a> {
    pub id: &'a str,
}

impl Command<()> for DeleteItem<'_> {
    fn execute(&self, ctx: &AppContext<impl ItemStore>) -> Result<(), AppError> {
        let id = ItemId::new(self.id)?;
        ctx.store().delete_item(&id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::MockItemStore;

    #[test]
    fn delete_item_forwards_to_store() {
        let store = MockItemStore::default();
        let ctx = AppContext::new(store);
        let command = DeleteItem { id: "demo" };

        command.execute(&ctx).expect("execution should succeed");

        let calls = ctx.store().delete_calls.borrow();
        assert_eq!(calls.as_slice(), ["demo".to_string()]);
    }

    #[test]
    fn delete_item_rejects_invalid_id() {
        let store = MockItemStore::default();
        let ctx = AppContext::new(store);
        let command = DeleteItem { id: "invalid/id" };

        let result = command.execute(&ctx);
        assert!(result.is_err());
    }
}
