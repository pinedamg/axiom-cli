# AXIOM: The Semantic Token Streamer 🦀

<p align="center">
  <img src="https://img.shields.io/badge/Rust-High%20Performance-orange?style=for-the-badge&logo=rust" alt="Rust">
  <img src="https://img.shields.io/badge/License-Apache%202.0-blue?style=for-the-badge" alt="License">
  <img src="https://img.shields.io/badge/Status-Alpha-yellow?style=for-the-badge" alt="Status">
  <img src="https://img.shields.io/badge/Savings-60--90%25-green?style=for-the-badge" alt="Savings">
</p>

<p align="center">
  <strong>"Stop burning 90% of your AI context window on terminal noise."</strong>
</p>

---

## 🚀 The AI Agent's Best Friend

Current AI agents like **Cursor**, **Claude Code**, or **Gemini CLI** are powerful but **context-hungry**. When you run a command like `npm install`, your terminal spits out thousands of tokens of progress bars and noise. 

**Axiom** is a high-performance proxy that acts as an intelligent **Semantic Firewall**. It intercepts your terminal stream, redacts secrets locally, collapses repetitive noise, and delivers only the **high-signal intent** your LLM needs.

### 💰 Why you need Axiom right now:
- **Instant Token Savings**: Cut your token usage by **60% to 90%** on every dev task.
- **Smarter AI Responses**: Give your LLM the signal, not the noise. No more missing errors buried in thousands of logs.
- **Privacy Shield**: Automatically redact API keys and PII **before** they leave your machine.
- **Ultra-Fast**: Written in Rust with sub-10ms overhead. You won't even know it's there.

---

## ⚡ The Axiom Effect (Live Demo)

### ❌ Without Axiom (~2,000 Tokens wasted)
```text
npm WARN deprecated inflight@1.0.6: ...
npm notice scanning for vulnerabilities...
fetch http://registry.npmjs.org/axios/-/axios-1.6.2.tgz
downloading [####################] 100%
added 124 packages in 5s...
(and 200+ more lines of progress noise...)
```

### ✅ With Axiom (~50 Tokens used)
```text
[AXIOM] ⚡ Collapsed 124 noise logs. High-signal stream active.
✔ Added 124 packages in 5s. 
[AXIOM] 🛡️ Privacy Shield: 0 secrets detected. 98% context window preserved.
```

---

## 🛠️ Get Started in 30 Seconds

### 1. Install
```bash
cargo install --git https://github.com/mpineda/axiom
axiom install
```

### 2. Profit
Just prefix any command. Axiom handles the rest.
```bash
axiom npm install
axiom docker-compose up
axiom git diff
```

---

## 📖 Documentation / Documentación

Choose your language for the deep dive:

### 🇺🇸 [English (EN)](docs/en/README.md)
- 🚀 **[Installation & Setup](docs/en/getting-started/installation.md)**
- ⚡ **[Quick Start Guide](docs/en/getting-started/quick-start.md)**
- 🛡️ **[Privacy & Telemetry](docs/en/user-guide/telemetry-and-privacy.md)**
- 🧩 **[WASM Plugin System](docs/en/developer-guide/plugins.md)**

### 🇪🇸 [Español (ES)](docs/es/README.md)
- 🚀 **[Instalación](docs/es/empezando/instalacion.md)**
- ⚡ **[Inicio Rápido](docs/es/empezando/inicio-rapido.md)**
- 🛡️ **[Privacidad Local](docs/es/guia-usuario/telemetria-y-privacidad.md)**

---

## 🤝 Community & Contributing

We're building the future of semantic CLI streaming. Whether you're adding [YAML schemas](docs/en/developer-guide/schemas.md) or core Rust logic, we'd love to have you.

Axiom Core is licensed under the **Apache License 2.0**.

---
<p align="center">
  <i>"From raw bytes to semantic intent."</i>
</p>
