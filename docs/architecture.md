# Architecture

## Intent

`rs-cli-tmpl` is a reference template for Rust command line tools. The current
template demonstrates a concept-owned boundary model with one sample concept,
`items`. The sample concept is illustrative. The architectural point is that
top-level modules identify owners, while internal submodules express specific
boundary roles.

This document describes the current implementation and the design intent it
expresses.

## Design axis

The template is organized by owning concept at the top level.

- `src/app` owns CLI handling, dependency wiring, and use-case orchestration
- `src/items` owns the sample concept end-to-end
- `src/error.rs` owns the application-wide error type

Boundary roles such as contract, validation, and storage implementation are
kept inside `src/items` instead of being promoted to top-level taxonomies.

## Current structure

```text
src/
  error.rs
  app/
    api.rs
    context.rs
    cli/
    commands/
  items/
    item_id.rs
    store.rs
    storage/
      filesystem_store.rs
      settings.rs
    testing.rs
  lib.rs
  main.rs
```

`src/lib.rs` is the public library surface and CLI entrypoint export.
`src/main.rs` is the binary entrypoint that delegates to the library.

## Ownership rules

### app/

`src/app` coordinates commands and dependency wiring.
It does not own item invariants or storage layout.

Current examples:

- `cli/` defines the command-line surface
- `commands/` executes use cases through injected contracts
- `api.rs` provides library-facing orchestration
- `context.rs` carries injected dependencies

### items/

`src/items` owns the sample item concept, including validation, dependency
contract, concrete storage, and concept-specific test support.

Current example:

- `item_id.rs` validates the `ItemId` invariant
- `store.rs` defines `ItemStore`
- `storage/filesystem_store.rs` implements `ItemStore` with filesystem I/O
- `storage/settings.rs` owns storage configuration used by that implementation
- `testing.rs` provides `MockItemStore` for unit tests

### error.rs

`src/error.rs` owns `AppError` when error semantics are application-wide.

## Dependency direction

Dependency flow remains inward toward concept contracts and validation.

```text
main -> lib -> app -> items::store + items::item_id
items::storage -> items::store + items::item_id
app::api -> items::storage for default wiring
lib -> app + items + error
```

`items` does not depend on `app` or CLI parsing.

## Sample concept scope

The `items` concept is intentionally small. It exists to make ownership rules
concrete.

- `ItemId` demonstrates a pure validated type
- `ItemStore` demonstrates a dependency contract
- `FilesystemItemStore` demonstrates an adapter implementation
- `add`, `list`, and `delete` demonstrate use-case orchestration

The template therefore teaches how a concept can own its internal boundaries
without introducing top-level `ports` or `adapters` buckets.

## Growth path

As a project grows, new concepts are added as new top-level owners beside
`items`.

```text
src/
  app/
  items/
  github/
  exchange/
  workflow/
```

Each concept can then define its own validated types, contracts,
implementations, and test support internally.

## Testing model

Tests follow the same explicit boundaries.

- unit tests live next to the modules they verify
- `src/items/testing.rs` provides a test double for command logic
- `tests/cli.rs` verifies the CLI boundary
- `tests/library.rs` verifies the public library boundary
- `tests/harness/` provides shared integration fixtures
