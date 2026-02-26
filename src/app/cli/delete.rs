use crate::app::api;
use crate::domain::AppError;

pub(super) fn run(id: &str) -> Result<(), AppError> {
    api::delete(id)?;
    println!("🗑️  Deleted item '{id}'");
    Ok(())
}
