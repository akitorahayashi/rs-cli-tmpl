//! Library entry point exposing the core command handlers.

pub mod adapters;
pub(crate) mod app;
pub mod domain;
pub mod ports;

#[cfg(test)]
pub(crate) mod testing;

pub use app::api::{add, delete, list};
pub use app::cli::run as cli;
pub use domain::AppError;
