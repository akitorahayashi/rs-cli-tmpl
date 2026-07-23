# rs-cli-tmpl Development Overview

## Project Summary

`rs-cli-tmpl` is a reference template for building Rust command line tools with
concept-owned module boundaries. It demonstrates how to separate orchestration
from concept ownership while keeping contracts and concrete implementations
inside the concept module. The template includes sample command families
(`item`, `label`, and `labeling`) that can be replaced or extended with
project-specific behavior.

## Documentation

- [Architecture](docs/architecture.md) — source layout, ownership boundaries,
  dependency direction, and growth path
- [Usage](README.md#usage) — sample command behavior and storage layout
- [Testing](README.md#testing-culture) — test boundaries and shared fixtures
- [Adapting the template](README.md#adapting-the-template) — the project areas
  replaced or extended for a concrete tool

Top-level owners: `cli/` for interface adaptation, `app/` for dependency wiring
and use-case orchestration, `items/` and `labels/` for the sample concepts, and
`error.rs` for application-wide errors.

## Verify Commands

```bash
just fix
just check
just test
```

Run `fix` before `check`; `check` does not modify files.
