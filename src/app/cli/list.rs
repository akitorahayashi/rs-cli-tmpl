use crate::AppError;
use crate::app::api;

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
