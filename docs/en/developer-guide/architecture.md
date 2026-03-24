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

The engine orchestrator (`src/engine/mod.rs`) processes each line through a strict pipeline:

1. **Stage 1: Structural Pre-processing**: Transforms lines resembling Markdown tables into actual Markdown format if the markdown mode is enabled.
2. **Stage 2: Resource Guarding**: Prevents buffer overflows or token burns by applying file length limits (e.g., summarizing after 100 lines) and auto-discovery noise checks.
3. **Stage 3: Security & Privacy**: The mandatory `PrivacyRedactor` step where sensitive patterns are scrubbed from the working line.
4. **Stage 4: Semantic Relevance**: Evaluates if the line is explicitly relevant to the current `IntentContext` (via intent priority overriding) and bypasses compression if true.
5. **Stage 5: Pattern-based Compression**: Matches the line against YAML `ToolSchema` rules. Lines are kept, collapsed into summaries, redacted by schema rules, or completely hidden.
6. **Stage 6: External Logic (WASM Plugins)**: Applies loaded WebAssembly plugins for external logic transformations.

## 4. Security Standards

- **Zero-Log Policy**: Raw captured data is **never** written to Axiom's own logs or telemetry.
- **Local-First**: All heavy lifting (Redaction, BERT embeddings, transformation) happens locally on the user's CPU.
