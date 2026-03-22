# AXIOM: The Semantic Token Streamer

[![Language](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-Private-red.svg)](#)
[![Status](https://img.shields.io/badge/status-Alpha-yellow.svg)](#)

> **"The best way to save tokens is to understand the intent behind them."**

**AXIOM** is a next-generation CLI proxy designed to transform raw terminal output into a condensed, high-signal semantic stream optimized for Large Language Models (LLMs). It effectively acts as an intelligent firewall and compressor between your terminal and AI agents like Gemini CLI, Claude Code, or Cursor.

---

## 🌟 Core Pillars

### 1. Privacy Shield (Blind-by-Design)
Security is our foundation. Axiom ensures that sensitive data never leaves your local machine.
- **Entropy-based Scanning**: Automatically detects high-entropy strings (API keys, secrets, tokens) using Shannon Entropy metrics.
- **PII Redaction**: Built-in regex engine to mask Emails, IP addresses, and other Personally Identifiable Information before any semantic processing occurs.

### 2. Intent-Aware Semantic Compression
Axiom doesn't just filter; it understands. It prioritizes information based on what you are actually trying to achieve.
- **Silent Discovery**: Automatically scans local chat logs (Gemini, Claude, Cursor) to extract your last prompt.
- **Intent Overriding**: If you ask about a specific error, Axiom will force-show relevant logs even if they match a "noise" pattern.

### 3. Structural Auto-Discovery
Zero-config learning for any tool.
- **Log Template Extraction**: Axiom identifies repetitive "skeletons" in logs (e.g., `Task #<NUM> processed in <TIME>`).
- **Zero-Trust Learning**: It learns the structure of unknown tools on-the-fly and stores them in a local SQLite database for future sessions.

### 4. Smart Aggregation
Move beyond simple line-dropping into intelligent synthesis.
- **Variable Capturing**: Instead of hiding 100 lines, Axiom provides a dense summary: `[AXIOM] 100 items processed successfully. IDs: [0x1, 0x2... 0x100]`.
- **Token Efficiency**: Achieve up to 90% token savings on noisy outputs without losing critical context.

---

## 🚀 Getting Started

### Installation
Ensure you have Rust and Cargo installed.
```bash
git clone git@github.com:pinedamg/axiom-cli.git
cd axiom-cli
cargo build --release
# Add to your path
cp target/release/axiom /usr/local/bin/
```

### Basic Usage
Use Axiom as a prefix for any command:
```bash
axiom npm install lodash
axiom git diff
axiom docker ps -a
```

### Invisible Intelligence (Auto-Context)
Axiom will automatically attempt to find your context. If you want to force a specific intent:
```bash
AXIOM_CONTEXT="Why is my database connection failing?" axiom ./run_app.sh
```

---

## 🛠️ Technical Architecture

Axiom follows **SOLID** and **Clean Architecture** principles to ensure low latency (<10ms overhead):

1.  **Gateway Layer**: Intercepts `stdout/stderr` streams using non-blocking asynchronous I/O.
2.  **Privacy Layer**: Local-first redaction engine.
3.  **Engine Layer**: The "Brain" that coordinates the Schema Matcher, Discovery Engine, and Intent Resolver.
4.  **Persistence Layer**: Local SQLite storage for historical memory and token saving analytics.

---

## ⚙️ Configuration

Axiom is designed to be plug-and-play, but highly configurable via `AxiomConfig`:
- **Schemas**: Add custom YAML rules in `config/schemas/`.
- **Intent Sources**: Configure custom paths for AI tool logs.
- **Entropy Threshold**: Adjust the sensitivity of the secret scanner.

---

## 🗺️ Roadmap

- [x] Phase 1: Security & Proxy Foundation.
- [x] Phase 2: Intent-Awareness & Persistence.
- [x] Phase 3: Auto-Discovery & Smart Aggregation.
- [ ] **Phase 3.2: WASM Plugin System** (Coming Soon).
- [ ] **Phase 4: Local AI Embeddings** (Vision).

---

## 📄 Documentation

- [AXIOM Manifesto](AXIOM_MANIFESTO.md): The vision and original thesis.
- [Roadmap](ROADMAP.md): Detailed development phases.
- [Development Log](docs/DEVELOPMENT_LOG.md): Architectural decisions and validation history.

---
© 2026 Axiom Dev Team. Private Repository.
