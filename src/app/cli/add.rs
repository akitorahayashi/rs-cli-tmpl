use crate::app::api;
use crate::domain::AppError;

pub(super) fn run(id: &str, content: &str) -> Result<(), AppError> {
    api::add(id, content)?;
    println!("✅ Added item '{id}'");
    Ok(())
}
