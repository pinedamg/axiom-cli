# AXIOM: System Architecture

This document describes the high-performance, layered architecture of **Axiom**. It is designed for minimal latency (<10ms) and maximum security while processing terminal streams.

## 1. High-Level Architecture

Axiom follows a **Layered Clean Architecture** adapted for Rust's performance needs. Data flows through a pipeline of specialized modules:

### 📥 1. Gateway (Ingress Layer)
- **Location**: `src/gateway/`
- **Responsibility**: Interacts with the Operating System. It captures `stdin`, `stdout`, and `stderr` from the child process.
- **Tech**: Uses `tokio::process` for non-blocking I/O.
- **PTY (Future)**: Plans to move from simple pipes to Pseudo-Terminals to preserve colors and interactivity.

### 🛡️ 2. Privacy (Firewall Layer)
- **Location**: `src/privacy/`
- **Responsibility**: The first point of processing. It ensures sensitive data never leaves the machine.
- **Mechanisms**: 
    - **Entropy Scanner**: Detects high-entropy strings (API keys, secrets) using Shannon Entropy metrics.
    - **Redactor**: Masks PII (Emails, IPs, etc.) before the next layer sees the data.

### 🧩 3. Schema (Domain Layer)
- **Location**: `src/schema/`
- **Responsibility**: Defines how to understand various CLI tools.
- **Logic**: Loads YAML files from `config/schemas/` and matches them against the current command.

### 🧠 4. Engine (Intelligence Layer)
- **Location**: `src/engine/`
- **Responsibility**: The orchestrator. It coordinates:
    - **Discovery**: Automatically identifies the tool and its intent.
    - **Intelligence**: Uses keyword, fuzzy, and neural (BERT-based) matching to determine relevance.
    - **Transformer**: Applies the transformation rules (Collapse, Drop, Pass).

### 📊 5. Persistence (Analytics Layer)
- **Location**: `src/persistence/`
- **Responsibility**: Local storage for token savings analytics and command history.
- **Tech**: SQLite for local, fast structured storage.

## 2. Technical Guidelines

- **Language**: Rust (Edition 2021).
- **Asynchrony**: `tokio` for high-concurrency non-blocking I/O.
- **Serialization**: `serde` for YAML and JSON handling.
- **Error Management**: `thiserror` for internal errors and `anyhow` for the CLI surface.

## 3. Data Flow (The Stream Pipeline)

1.  **Command Execution**: `axiom npm install` starts.
2.  **Process Detective**: Identifies `npm` and the current project context.
3.  **Stream Capture**: Raw bytes are read from the sub-process.
4.  **Deduplicate**: The engine checks for repeated lines and collapses them.
5.  **Transform**: Content like tables is converted to Markdown format.
6.  **Guard**: Thresholds are checked to prevent terminal flooding (Guardian Mode).
7.  **Redact**: Lines are scanned and secrets/PII are redacted.
8.  **Analyze**: Schema logic and Semantic matches process the cleaned line.
9.  **Plugins**: Any WebAssembly plugins perform final manipulations.
10. **Final Output**: High-signal output is printed to the terminal for the AI agent to consume.
11. **Analytics**: Savings are calculated and stored in the local SQLite DB.

## 4. Security Standards

- **Zero-Log Policy**: Raw captured data is **never** written to Axiom's own logs or telemetry.
- **Local-First**: All heavy lifting (Redaction, BERT embeddings, transformation) happens locally on the user's CPU.

## 5. Developer Laboratory Mode (Decision Tracing)

Axiom includes a built-in Developer Laboratory Mode (`axiom dev <args>...` or `--dev`), designed to expose the internal decision-making process of the pipeline.

When Developer Laboratory Mode is enabled, Axiom intercepts the existing six pipeline stages (Deduplicate, Transform, Guard, Redact, Analyze, Plugins) and records every mutation or decision made to the terminal output. It generates a comprehensive trace report, explaining why a line was kept, collapsed, or redacted, directly in the terminal, allowing schema creators to easily debug their rules.
