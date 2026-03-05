# ==============================================================================
# justfile for rs-cli-tmpl development
# ==============================================================================

set shell := ["bash", "-eu", "-o", "pipefail", "-c"]

# Show available recipes
default: help

# Show available recipes
help:
    @echo "Usage: just [recipe]"
    @echo ""
    @echo "Development tasks for rs-cli-tmpl:"
    @just --list | tail -n +2 | awk '{printf "  \033[36m%-20s\033[0m %s\n", $1, substr($0, index($0, $2))}'

# ==============================================================================
# Environment Setup
# ==============================================================================

# Initialize project: install dependencies and configure hooks
setup:
    @echo "🪄 Installing tools with mise..."
    @mise trust
    @mise install --locked
    @echo "🪝 Configuring git hooks..."
    chmod +x .githooks/pre-commit
    git config core.hooksPath .githooks

# ==============================================================================
# Lint & Format
# ==============================================================================

# Format code
fix:
    cargo fmt
    just --fmt --unstable

# Verify formatting, lint, and compilation
check:
    cargo check
    cargo fmt --check
    cargo clippy --all-targets --all-features -- -D warnings
    just --fmt --check --unstable

# ==============================================================================
# Testing
# ==============================================================================

# Run all tests
test:
    cargo test --all-targets --all-features

# Generate code coverage report
coverage:
    rm -rf target/tarpaulin coverage
    env -u RUSTC_WRAPPER -u SCCACHE_IGNORE_SERVER_IO_ERROR -u SCCACHE_ERROR_LOG mise exec -- cargo tarpaulin --engine llvm --out Xml --output-dir coverage --all-features --fail-under 30

# ==============================================================================
# Build Tasks
# ==============================================================================

# Compile the project
build:
    cargo build
