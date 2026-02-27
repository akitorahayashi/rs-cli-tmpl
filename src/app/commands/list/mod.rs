use crate::app::AppContext;
use crate::domain::{AppError, ports::ItemStore};

/// List all item identifiers from storage.
pub fn execute(ctx: &AppContext<impl ItemStore>) -> Result<Vec<String>, AppError> {
    ctx.store().list_items()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::MockItemStore;

    #[test]
    fn list_items_returns_store_values() {
        let store = MockItemStore::default();
        store.set_list_items(["first", "second"]);
        let ctx = AppContext::new(store);

        let items = execute(&ctx).expect("execution should succeed");
        assert_eq!(items, vec!["first".to_string(), "second".to_string()]);
    }
}
