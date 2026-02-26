use crate::app::api;
use crate::domain::AppError;

pub(super) fn run() -> Result<(), AppError> {
    let items = api::list()?;

    println!("📦 Stored items:");
    if items.is_empty() {
        println!("(none)");
    } else {
        for id in &items {
            println!("- {id}");
        }
    }

    Ok(())
}
