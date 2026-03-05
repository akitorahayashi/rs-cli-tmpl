# rs-cli-tmpl

`rs-cli-tmpl` is a reference template for building Rust-based command line tools with a clean,
layered architecture. It demonstrates how to separate concerns across the CLI interface,
application commands, pure business logic, and I/O abstractions so new projects can start from a
well-tested foundation.

## Architectural Highlights

- Layered architecture: `domain/` contains pure invariants, `ports/` defines trait
  boundaries, `adapters/` provides implementations, and `app/` separates API orchestration,
  CLI adaptation, and command execution with `AppContext`.
- I/O abstraction: `src/ports/item_store.rs` defines an `ItemStore` trait and
  `src/adapters/filesystem_item_store.rs` implements it, rooted at `~/.config/rs-cli-tmpl`.
- Configuration management: `src/adapters/storage_settings.rs` provides storage path
  configuration, enabling easy testing with custom paths.
- Robust testing strategy: unit tests live next to their modules, `src/testing/`
  provides a `MockItemStore` for command logic tests (with `#[cfg(test)]`), and the `tests/`
  directory provides integration suites for both the library API and the CLI binary.

The template ships with minimal sample commands (`add`, `list`, and `delete`) that show how to
thread dependencies through each layer. Replace or extend them with your own domain logic while
reusing the same structure.

## Storage Layout

The template stores items under `~/.config/rs-cli-tmpl/<id>/item.txt`. For example, after running `rs-cli-tmpl add my-item --content '...'`:

```text
~/.config/rs-cli-tmpl/
  my-item/
    item.txt
```

## Quick Start

```bash
cargo install --path .
# or
cargo build --release
```

The optimized binary will be created at `target/release/rs-cli-tmpl`.

## Usage

```bash
rs-cli-tmpl --version    # Show version information
rs-cli-tmpl add <id>     # Add an item
rs-cli-tmpl list         # List items
rs-cli-tmpl delete <id>  # Delete an item
```

## Release Provenance and Verification

The release workflow publishes these artifacts for every platform binary:

- `*.sig`: Sigstore keyless signature
- `*.bundle`: Sigstore verification bundle
- GitHub build provenance attestation (SLSA)

Consumers can choose whether to enforce verification, but release metadata is always published so
the choice is available.

Example verification for a downloaded binary:

```bash
gh release verify-asset <tag> <asset-path> --repo <owner>/<repo>
```

## Development Commands

- `just setup`: install pinned development tools from `mise.toml`.
- `cargo build`: build a debug binary.
- `cargo build --release`: build the optimized release binary.
- `cargo fmt`: format code using rustfmt.
- `cargo fmt --check && cargo clippy --all-targets --all-features -- -D warnings`: format check and lint with clippy.
- `cargo test --all-targets --all-features`: run all tests.
- `just coverage`: run coverage with pinned tarpaulin.
- `cargo fetch --locked`: pre-fetch dependencies.

## Testing Culture

- Unit Tests: Live alongside their modules inside `src/`, covering helper utilities and
  domain invariants.
- Command Logic Tests: Use the mock store in `src/testing/mock_item_store.rs` (conditionally
  compiled with `#[cfg(test)]`) to exercise command implementations without touching the filesystem.
- Integration Tests: Located in the `tests/` directory with explicit executable boundaries:
  `tests/cli.rs` for CLI flows and `tests/library.rs` for public API behavior. Behavior-oriented
  modules live under `tests/cli/` and `tests/library/`; shared fixtures live in
  `tests/harness/test_context.rs`.

## Project Structure

```text
rs-cli-tmpl/
├── src/
│   ├── main.rs                # Binary entrypoint delegating to library CLI
│   ├── lib.rs                 # Public API + CLI entrypoint re-exports
│   ├── app/                   # Application layer
│   │   ├── api.rs             # Library-facing use-case orchestration
│   │   ├── context.rs         # AppContext
│   │   ├── cli/               # CLI adapter (clap + output formatting)
│   │   │   ├── mod.rs
│   │   │   ├── add.rs
│   │   │   ├── list.rs
│   │   │   └── delete.rs
│   │   └── commands/          # Use-case command execution modules
│   │       ├── add/mod.rs
│   │       ├── list/mod.rs
│   │       └── delete/mod.rs
│   ├── domain/                # Pure business invariants
│   │   ├── error.rs           # AppError definitions
│   │   └── item_id.rs         # ItemId validation
│   ├── ports/                 # Trait boundaries
│   │   └── item_store.rs      # ItemStore trait
│   ├── adapters/              # I/O implementations
│   │   ├── filesystem_item_store.rs
│   │   └── storage_settings.rs
│   └── testing/               # Test infrastructure (#[cfg(test)])
│       └── mock_item_store.rs
├── tests/
│   ├── cli.rs                 # Integration test target: CLI boundary
│   ├── library.rs             # Integration test target: public API boundary
│   ├── cli/                   # CLI behavior specs
│   ├── library/               # Library behavior specs
│   └── harness/               # Shared integration fixtures
└── docs/
    └── architecture/
        └── ARCHITECTURE_BOUNDARY.md
```

## Adapting the Template

1. Replace the sample commands in `src/app/commands/<command>/` with your own business logic.
2. Extend `src/app/api.rs` to compose dependencies and expose use-case APIs.
3. Update the CLI definitions in `src/app/cli/` to match your command surface.
4. Refresh the integration tests and documentation to describe the new behavior.

Happy hacking!
