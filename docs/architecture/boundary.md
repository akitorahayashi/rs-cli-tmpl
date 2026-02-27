# Architecture Boundary

## Purpose

`rs-cli-tmpl` uses a layered boundary model that separates pure domain logic from
application orchestration and external I/O.

This document describes the current boundary contracts across `src/` and
`tests/`.

## Layer Map

### `src/domain`

- Owns pure business invariants, domain error semantics, and port contracts.
- Contains no filesystem, environment, process, or network access.
- Current modules:
  - `item_id.rs`
  - `error.rs`
  - `ports/item_store.rs` (`ItemStore` trait)

### `src/adapters`

- Implements `ports` using concrete external mechanisms.
- Owns filesystem access and environment-based configuration resolution.
- Current modules:
  - `filesystem_item_store.rs`
  - `storage_settings.rs`

### `src/app`

- Orchestrates use cases by combining domain rules with port-backed dependencies.
- Encodes command-level behavior without direct external I/O implementation.
- Current modules:
  - `api.rs`
  - `cli/*`
  - `context.rs`
  - `commands/<command>/mod.rs`

### `src/lib.rs` and `src/main.rs`

- `lib.rs` re-exports the public API surface and CLI entrypoint.
- `main.rs` delegates to the library CLI entrypoint.

## Dependency Direction

Dependencies flow inward toward stable business rules:

`main` -> `lib` -> `app` -> `domain::ports` -> `domain`
`app` -> `domain`
`adapters` -> `domain::ports` + `domain`
`lib` -> `adapters` for default wiring

`domain` does not depend on `app`, `adapters`, or CLI parsing.

## Boundary Rules

### Domain Boundary

- Domain types remain runtime-agnostic.
- Domain validation errors are explicit and deterministic.

### Port Boundary

- Ports define capabilities required by use cases, not storage details.
- Port methods use domain types (`ItemId`, `AppError`) as boundary contracts.

### Adapter Boundary

- Adapters isolate path layout, environment lookup, and filesystem mutations.
- Adapter-specific configuration remains outside `domain`.

### Application Boundary

- Commands call ports through `AppContext`.
- Application code does not import `std::fs` for persistence behavior.

## Placement Guide

- `src/domain` houses business invariants and validation logic.
- `src/app/commands/<command>/mod.rs` contains use-case orchestration.
- `src/app/api.rs` defines library-facing orchestration entry points.
- `src/domain/ports` holds dependency contracts.
- `src/adapters` manages I/O implementations and env/path resolution.
- `src/app/cli/` structures CLI argument surfaces and output shaping.
- `src/app/api.rs` and `src/lib.rs` provide reusable library entry points.

## Test Boundary Model

Integration tests are split into explicit executable targets:

- `tests/cli.rs` for CLI behavior contracts
- `tests/library.rs` for public API behavior contracts
- `tests/harness/` for shared fixture utilities

Behavior modules are grouped under `tests/cli/` and `tests/library/` using
explicit behavior names.
