# AXIOM: The Semantic Token Streamer

> **Vision**: Transform the CLI from a raw byte stream into an intent-aware semantic layer for LLMs.

**AXIOM** is a token compression tool designed to optimize the interaction between developers and LLMs in the terminal.

## 🚀 Documentation

- [AXIOM Manifesto](AXIOM_MANIFESTO.md): The why and the pillars of the project.
- [Roadmap](ROADMAP.md): Development phases and long-term goals.
- [Phase 1 Implementation Guide](docs/PHASE_1_GUIDE.md): Technical guide for starting development (Alpha).

## 🛠️ Tech Stack

- **Core**: Rust (Tokio for async I/O).
- **Security**: Entropy-based scanning and PII redaction.
- **Intelligence**: Intent-Aware filtering (integrating user prompt context).
- **Data**: SQLite for telemetry and caching.

## ⚙️ Initial Configuration

To compile the project (once logic is implemented):
```bash
cargo build --release
```

To run tests:
```bash
cargo test
```

---
*"The best way to save tokens is to understand the intent behind them."*
