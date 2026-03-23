# AXIOM: The Semantic Token Streamer 🦀

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-Alpha-yellow.svg)](#)
[![Speed](https://img.shields.io/badge/overhead-%3C10ms-green.svg)](#)

> **"Stop wasting 90% of your context window on terminal noise."**

**Axiom** is a high-performance CLI proxy written in Rust that acts as an intelligent "Semantic Firewall" between your terminal and AI agents (Cursor, Claude Code, Gemini CLI). It transforms raw, noisy command outputs into a condensed, high-signal stream optimized for LLMs.

---

## 🚀 Why Axiom?

Current AI agents are powerful, but they are "context-hungry." When you run a command like `npm install` or `docker logs`, 90% of the output is repetitive noise that:
1.  **Drains your wallet:** You pay for thousands of unnecessary tokens.
2.  **Loses context:** Critical errors get buried under thousands of lines of successful logs.
3.  **Leaks Secrets:** Sensitive data (API keys, PII) can be sent to LLM providers accidentally.

**Axiom fixes this locally, in real-time, with sub-10ms overhead.**

---

## ✨ Key Features

### 🛡️ Privacy Shield (Local-First)
Axiom ensures sensitive data never leaves your machine.
- **Entropy Scanning**: Automatically detects and redacts high-entropy strings (API keys, secrets) using Shannon Entropy metrics.
- **PII Redaction**: Built-in engine to mask emails, IPs, and sensitive patterns before they reach the AI.

### 🧠 Intent-Aware Compression
Axiom doesn't just filter; it understands. 
- **Smart Aggregation**: Compresses 100+ lines of success into a single dense summary: `[AXIOM] 124 items processed successfully. IDs: [0x1...0x7B]`.
- **Intent Overriding**: If you are debugging a specific error, Axiom force-shows relevant logs while suppressing the rest.

### ⚡ Built for Speed
- **Native Rust**: Zero-cost abstractions and non-blocking I/O.
- **WASM Plugin System**: Extend Axiom with secure, portable plugins.

---

## 📊 Token Savings Analytics

Axiom tracks your savings in a local SQLite database. 
```bash
axiom gain --history
```
*Typical results show **60% to 90% reduction** in token usage for common dev tasks.*

---

## 🛠️ Installation

```bash
git clone https://github.com/pinedamg/axiom.git
cd axiom
cargo build --release
cp target/release/axiom /usr/local/bin/
```

### Usage
Simply prefix any command:
```bash
axiom npm install
axiom git diff
axiom docker-compose up
```

---

## 🤝 Contributing & License

We welcome contributions! Please read our [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the CLA.

Axiom Core is licensed under the **Apache License 2.0**.

---
*“From raw bytes to semantic intent.”*
