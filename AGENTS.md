# rs-cli-tmpl Development Overview

## Project Summary
`rs-cli-tmpl` is a reference template for building Rust-based command line tools with a clean, layered architecture. It demonstrates how to separate concerns across domain invariants, ports, services, and application commands, providing a well-tested foundation for new projects. The template includes sample commands (`add`, `list`, and `delete`) that can be replaced or extended with custom domain logic.

## Tech Stack
- **Language**: Rust
- **CLI Parsing**: `clap`
- **Development Dependencies**:
  - `assert_cmd`
  - `assert_fs`
  - `predicates`
  - `serial_test`
  - `tempfile`

## Coding Standards
- **Formatter**: `rustfmt` is used for code formatting. Key rules include a maximum line width of 100 characters, crate-level import granularity, and grouping imports by standard, external, and crate modules.
- **Linter**: `clippy` is used for linting, with a strict policy of treating all warnings as errors (`-D warnings`).

## Naming Conventions
- **Structs and Enums**: `PascalCase` (e.g., `Cli`, `Commands`)
- **Functions and Variables**: `snake_case` (e.g., `run_tests`, `test_context`)
- **Modules**: `snake_case` (e.g., `cli_commands.rs`)

## Verify Commands
- **Format**: `cargo fmt --check`
- **Lint**: `cargo clippy --all-targets --all-features -- -D warnings`
- **Test**: `cargo test --all-targets --all-features`

## Testing Strategy
- **Unit Tests**: Located within the `src/` directory alongside the code they test, covering domain invariants and service implementations.
- **Command Logic Tests**: Found in `src/app/commands/`, utilizing `MockItemStore` from `src/testing/` (compiled with `#[cfg(test)]`) to ensure business logic is tested in isolation via the `Command` trait.
- **Integration Tests**: Housed in the `tests/` directory, organised into two explicit boundaries: `tests/cli.rs` for CLI user flows and `tests/library.rs` for the public library API. Behavior-oriented modules live under `tests/cli/` and `tests/library/`; shared fixtures reside in `tests/harness/test_context.rs`.
- **CI**: GitHub Actions automatically runs build, linting, and test workflows, as defined in `.github/workflows/`.

## Architectural Highlights
- **Layered architecture**: `domain/` contains pure invariants (no I/O), `ports/` defines trait boundaries, `services/` provides implementations, and `app/` wires commands with `AppContext`.
- **I/O abstraction**: `src/ports/item_store.rs` defines an `ItemStore` trait and `src/services/filesystem_item_store.rs` implements it, rooted at `~/.config/rs-cli-tmpl`.
- **Configuration management**: `src/services/storage_settings.rs` provides storage path configuration, enabling custom paths for testing.
- **Storage Layout**: Items are stored under `~/.config/rs-cli-tmpl/<id>/item.txt`.
