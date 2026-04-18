use crate::AppError;
use crate::app::api;

pub(super) fn run(id: &str) -> Result<(), AppError> {
    api::delete(id)?;
    println!("🗑️  Deleted item '{id}'");
    Ok(())
}
