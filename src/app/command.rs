use crate::domain::AppError;
use crate::ports::ItemStore;

use super::AppContext;

/// Trait for application commands that operate on the app context.
pub trait Command<R> {
    fn execute(&self, ctx: &AppContext<impl ItemStore>) -> Result<R, AppError>;
}
