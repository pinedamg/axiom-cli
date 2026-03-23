# Installation & Setup

## Prerequisites

- **Rust / Cargo**: You need Rust installed to compile Axiom from source. If you don't have it, install it from [rustup.rs](https://rustup.rs/).

## Installing Axiom

You can install Axiom directly from the GitHub repository using Cargo:

```bash
cargo install --git https://github.com/mpineda/axiom
```

After installation, run the setup command to initialize Axiom's configuration, local database, and schemas:

```bash
axiom install
```

## Verifying Installation

Verify that Axiom was installed correctly:

```bash
axiom --version
```
This should return the current version of Axiom.

Check your current configuration and telemetry status:
```bash
axiom status
```
