# AXIOM: Phase 1 Implementation Guide (Alpha)

This document describes the technical architecture and code standards for the first phase of **AXIOM**.

## 1. System Architecture

We will follow a **Layered Architecture (Clean Architecture)** adapted to Rust's performance needs:

1.  **Gateway (Interface Layer)**:
    - Responsible for interacting with the operating system.
    - Captures `stdin`, `stdout`, and `stderr`.
    - Manages sub-process execution using `tokio::process`.

2.  **Privacy (Firewall Layer)**:
    - First point of processing for raw data.
    - Implements a **Shannon Entropy** scanner to identify secrets.
    - Filters PII (Personally Identifiable Information) before any other layer sees the data.

3.  **Schema (Domain Layer)**:
    - Defines transformation rules and data models.
    - Loads YAML configurations and compiles them into efficient memory representations.

4.  **Engine (Core Layer)**:
    - The system orchestrator. Coordinates data input, cleaning, and final transformation.

## 2. Technical Guidelines

- **Language**: Rust (Edition 2021).
- **Asynchrony**: `tokio` for non-blocking I/O.
- **Serialization**: `serde` for handling YAML and JSON.
- **Error Management**: `thiserror` for internal errors and `anyhow` for the CLI.

## 3. Proxy (Gateway) Implementation

The heart of Phase 1 is the interceptor. A proxy command must be implemented that:
1.  Captures the user's command (e.g., `axiom git status`).
2.  Starts the sub-process (e.g., `git status`).
3.  Reads the output stream in chunks.
4.  Passes each chunk through the **Privacy Scanner** before sending it to the terminal/agent.

## 4. Security Standards (Mandatory)

- **Zero Leaks**: No data captured from `stdout` should be logged in Axiom's own log system.
- **Dynamic Entropy**: The entropy threshold for detecting secrets must be configurable but strict by default (e.g., > 4.5 bits/character).

## 5. Next Steps (Technical Checklist)

- [ ] Define the folder structure in `src/`:
  - `src/gateway/`
  - `src/privacy/`
  - `src/schema/`
  - `src/engine/`
- [ ] Implement the base CLI proxy logic in `main.rs`.
- [ ] Create the first prototype of the entropy engine.
