# rs-cli-tmpl Development Overview

## Project Summary
`rs-cli-tmpl` is a reference template for building Rust command line tools with
concept-owned module boundaries. It demonstrates how to separate orchestration
from concept ownership while keeping contracts and concrete implementations
inside the concept module. The template includes sample commands (`add`,
`list`, and `delete`) that can be replaced or extended with project-specific
behavior.

## Tech Stack
- Language: Rust
- CLI parsing: `clap`
- Development dependencies:
  - `assert_cmd`
  - `assert_fs`
  - `predicates`
  - `serial_test`
  - `tempfile`

## Coding Standards
- Formatter: `rustfmt` with a maximum line width of 100 characters,
  crate-level import granularity, and grouped imports.
- Linter: `clippy` with warnings treated as errors (`-D warnings`).

## Naming Conventions
- Structs and enums: `PascalCase`.
- Functions and variables: `snake_case`.
- Modules: `snake_case`.

## Verify Commands
- Format: `cargo fmt --check`
- Lint: `cargo clippy --all-targets --all-features -- -D warnings`
- Test: `cargo test --all-targets --all-features`

## Testing Strategy
- Unit tests: located in `src/` next to owned modules.
- Command logic tests: in `src/app/commands/` using `MockItemStore` from
  `src/items/testing.rs` under `#[cfg(test)]`.
- Integration tests: in `tests/`, with `tests/cli.rs` for CLI boundary behavior
  and `tests/library.rs` for public library boundary behavior. Shared fixtures
  live in `tests/harness/test_context.rs`.
- CI: GitHub Actions runs build, linting, and tests.

## Architectural Highlights
- Top-level owners: `app/` for orchestration, `items/` for the sample concept,
  and `error.rs` for application-wide errors.
- Contract and implementation co-location: `src/items/store.rs` defines
  `ItemStore`; `src/items/storage/filesystem_store.rs` implements it.
- Configuration ownership: `src/items/storage/settings.rs` provides storage path
  configuration for item persistence.
- Storage layout: items are stored under
  `~/.config/rs-cli-tmpl/<id>/item.txt`.
