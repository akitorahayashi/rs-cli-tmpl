use crate::app::{AppContext, Command};
use crate::domain::{AppError, ItemId};
use crate::ports::ItemStore;

/// Command to add an item to storage.
pub struct AddItem<'a> {
    pub id: &'a str,
    pub content: &'a str,
}

impl Command<()> for AddItem<'_> {
    fn execute(&self, ctx: &AppContext<impl ItemStore>) -> Result<(), AppError> {
        let id = ItemId::new(self.id)?;
        ctx.store().add_item(&id, self.content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::MockItemStore;

    #[test]
    fn add_item_forwards_to_store() {
        let store = MockItemStore::default();
        let ctx = AppContext::new(store);
        let command = AddItem { id: "demo", content: "example" };

        command.execute(&ctx).expect("execution should succeed");

        let calls = ctx.store().add_calls.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0], ("demo".to_string(), "example".to_string()));
    }

    #[test]
    fn add_item_rejects_invalid_id() {
        let store = MockItemStore::default();
        let ctx = AppContext::new(store);
        let command = AddItem { id: "invalid/id", content: "example" };

        let result = command.execute(&ctx);
        assert!(result.is_err());
    }
}
