pub mod attach;
pub mod detach;
pub mod find;
pub mod list;

use crate::AppError;
use crate::app::AppContext;
use crate::items::ItemStore;
use crate::labels::{LabelName, LabelStore};

pub(super) fn ensure_item_exists(
    ctx: &AppContext<impl ItemStore, impl LabelStore>,
    item_id: &str,
) -> Result<(), AppError> {
    if ctx.item_store().list_items()?.iter().any(|existing| existing == item_id) {
        Ok(())
    } else {
        Err(AppError::ItemNotFound(item_id.to_string()))
    }
}

pub(super) fn ensure_label_exists(
    ctx: &AppContext<impl ItemStore, impl LabelStore>,
    label_name: &LabelName,
) -> Result<(), AppError> {
    if ctx.label_store().list_labels()?.iter().any(|existing| existing == label_name.as_str()) {
        Ok(())
    } else {
        Err(AppError::LabelNotFound(label_name.as_str().to_string()))
    }
}
