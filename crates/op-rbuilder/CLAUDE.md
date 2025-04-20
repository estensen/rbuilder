# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build/Test Commands
- Build: `cargo build -p op-rbuilder [--bin op-rbuilder|tester] [--features flashblocks]`
- Run: `cargo run -p op-rbuilder --bin op-rbuilder -- node [OPTIONS]`
- Run tester: `cargo run -p op-rbuilder --bin tester -- [COMMAND]`
- Tests: `cargo test -p op-rbuilder [--features integration]`
- Single test: `cargo test -p op-rbuilder [--features integration] -- test_name`

## Code Style Guidelines
- Use standard Rust formatting conventions
- Organize imports: standard library first, then external crates, then local modules
- Error handling: use `eyre::Result<T>` for public APIs, `thiserror` for error definitions
- Types: leverage Rust's type system; prefer strong typing over primitive types
- Async: use `tokio` with `async/await` for asynchronous code
- Comments: document public APIs and complex implementations
- Naming: use snake_case for functions/variables, CamelCase for types/traits
- Use `tracing` for logging at appropriate levels
- Support metrics for observability

## Workspace Structure
- Crate is part of a workspace with shared dependencies and versions in root Cargo.toml
- Features toggle functionality (jemalloc, log levels, integration, flashblocks)