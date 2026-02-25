# Architecture Boundary

## Purpose

`rs-cli-tmpl` uses a layered boundary model that separates pure domain logic from
application orchestration and external I/O.

This document describes the current boundary contracts across `src/` and
`tests/`.

## Layer Map

### `src/domain`

- Owns pure business invariants and domain error semantics.
- Contains no filesystem, environment, process, or network access.
- Current modules:
  - `item_id.rs`
  - `error.rs`

### `src/ports`

- Defines boundary interfaces consumed by the application layer.
- Current module:
  - `item_store.rs` (`ItemStore` trait)

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
  - `context.rs`
  - `command.rs`
  - `commands/*`

### `src/lib.rs` and `src/main.rs`

- `lib.rs` exposes the public API and composes default dependencies.
- `main.rs` is the CLI entrypoint and translates CLI input into library calls.

## Dependency Direction

Dependencies flow inward toward stable business rules:

`main` -> `lib` -> `app` -> `ports` -> `domain`
`app` -> `domain`
`adapters` -> `ports` + `domain`
`lib` -> `adapters` for default wiring

`domain` does not depend on `app`, `ports`, `adapters`, or CLI parsing.

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

- New business invariants and validation logic: `src/domain`.
- New use-case orchestration: `src/app/commands`.
- New dependency contracts: `src/ports`.
- New I/O implementations or env/path resolution: `src/adapters`.
- New CLI argument surfaces: `src/main.rs`.
- New reusable library entry points: `src/lib.rs`.

## Test Boundary Model

Integration tests are split into explicit executable targets:

- `tests/cli.rs` for CLI behavior contracts
- `tests/library.rs` for public API behavior contracts
- `tests/harness/` for shared fixture utilities

Behavior modules are grouped under `tests/cli/` and `tests/library/` using
explicit behavior names.
