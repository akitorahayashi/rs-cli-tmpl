# rs-cli-tmpl

`rs-cli-tmpl` is a reference template for building Rust command line tools with
concept-owned module boundaries. It demonstrates how to keep orchestration in
`app/` while keeping concept rules, contracts, and concrete implementations
together under a single owning concept module.

## Architectural Highlights

- Concept ownership at the top level: `items/` owns the sample concept, while
  `app/` owns command orchestration and interface wiring.
- Internal boundaries inside the concept: `src/items/store.rs` defines the
  `ItemStore` contract and `src/items/storage/filesystem_store.rs` provides the
  filesystem implementation rooted at `~/.config/rs-cli-tmpl`.
- Concept-local configuration: `src/items/storage/settings.rs` resolves storage
  settings used by the item store.
- Stable application-wide error at root: `src/error.rs` defines `AppError`.
- Robust testing strategy: unit tests live next to their modules,
  `src/items/testing.rs` provides `MockItemStore` for command tests (compiled
  with `#[cfg(test)]`), and the `tests/` directory provides integration suites
  for both the library API and the CLI binary.

The template ships with minimal sample commands (`add`, `list`, and `delete`)
that show dependency flow across these boundaries. Replace or extend the sample
concept while keeping the same ownership model.

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
  concept invariants.
- Command Logic Tests: Use the mock store in `src/items/testing.rs` (conditionally compiled
  with `#[cfg(test)]`) to exercise command implementations without touching the filesystem.
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
│   ├── error.rs               # Application-wide AppError type
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
│   └── items/                 # Sample concept owner
│       ├── mod.rs
│       ├── item_id.rs         # ItemId validation
│       ├── store.rs           # ItemStore contract
│       ├── testing.rs         # MockItemStore for unit tests
│       └── storage/
│           ├── filesystem_store.rs
│           └── settings.rs
├── tests/
│   ├── cli.rs                 # Integration test target: CLI boundary
│   ├── library.rs             # Integration test target: public API boundary
│   ├── cli/                   # CLI behavior specs
│   ├── library/               # Library behavior specs
│   └── harness/               # Shared integration fixtures
└── docs/
    └── architecture.md
```

## Adapting the Template

1. Replace the sample commands in `src/app/commands/<command>/` with your own business logic.
2. Extend `src/app/api.rs` to compose dependencies and expose use-case APIs.
3. Update the CLI definitions in `src/app/cli/` to match your command surface.
4. Add new concept owners as siblings to `src/items/` when your project grows.
5. Refresh the integration tests and documentation to describe the new behavior.
