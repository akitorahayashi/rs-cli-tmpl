use crate::AppError;
use crate::app::api;

pub(super) fn run(id: &str, content: &str) -> Result<(), AppError> {
    api::add(id, content)?;
    println!("✅ Added item '{id}'");
    Ok(())
}
