use crate::items::ItemStore;

/// Application context holding dependencies for command execution.
pub struct AppContext<S: ItemStore> {
    store: S,
}

impl<S: ItemStore> AppContext<S> {
    /// Create a new application context with the given store.
    pub fn new(store: S) -> Self {
        Self { store }
    }

    /// Get a reference to the item store.
    pub fn store(&self) -> &S {
        &self.store
    }
}
