//! Library entry point exposing the core command handlers.

pub(crate) mod app;
mod error;
pub mod items;

pub use app::api::{add, delete, list};
pub use app::cli::run as cli;
pub use error::AppError;
