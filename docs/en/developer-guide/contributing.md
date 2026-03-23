# Contributing to Axiom

We welcome contributions from the community! Whether you want to add a new schema, fix a bug, or write a WASM plugin, your help is appreciated.

## Getting Started

1. Fork the repository.
2. Clone your fork locally.
3. Make sure you have Rust installed (`rustup`).
4. Run `cargo build` to ensure everything compiles.
5. Run `cargo test` to execute the test suite.

## Ways to Contribute

### 1. Adding Schemas
The easiest way to contribute is by adding support for new CLI tools. Read the [Creating Schemas](schemas.md) guide to learn how to create YAML definitions that filter noise from your favorite tools.

### 2. Core Development
If you want to work on the Rust core (e.g., the telemetry engine, privacy shield, or intent discovery), check out the [Architecture Guide](architecture.md) first to understand the layered design.

### 3. WASM Plugins
Axiom supports complex parsing via WebAssembly. See the [WASM Plugin Guide](plugins.md) for details on the ABI and how to write a plugin.

## Code of Conduct & CLA

Please review the primary `CONTRIBUTING.md` file in the root of the repository for legal and behavioral guidelines. All contributions must adhere to the Apache License 2.0.
