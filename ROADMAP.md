# AXIOM: Development Roadmap

This Roadmap defines the implementation path for **AXIOM**, prioritizing security and minimal latency.

## Phase 1: Foundation & Security (Alpha) - [CURRENT]
**Goal**: Build the flow interceptor and the data protection system.

### 1.1 Gateway Interceptor
- [ ] Implement the CLI proxy (`axiom <cmd>`).
- [ ] Non-blocking capture of `stdout` and `stderr`.
- [ ] Initial latency budget: <5ms overhead.

### 1.2 Privacy Layer (The Shield)
- [ ] Implement **Shannon Entropy** scanner to detect secrets (API Keys, hashes).
- [ ] Regex-based redaction engine for PII (Personally Identifiable Information).
- [ ] "Blind-by-Design" mode: Local redaction before any semantic processing.

### 1.3 Static YAML Schemas
- [ ] Basic YAML-based rule engine for common tools (`git`, `docker`, `npm`).
- [ ] Support for fixed and variable text blocks.

---

## Phase 2: Intelligence & Context (Beta)
**Goal**: Make Axiom understand the "why" behind command execution.

### 2.1 Intent-Aware Engine
- [ ] Integration with LLM chat history (last message).
- [ ] Extraction of key entities from the prompt (e.g., "errors", "diff", "logs").

### 2.2 Semantic Transformer
- [ ] Dynamic collapse/expansion logic based on "Intent".
- [ ] Flow prioritization: If the LLM is looking for errors, expand error traces and collapse warnings.

### 2.3 Persistence & Analytics
- [ ] SQLite integration for local history.
- [ ] Token saving telemetry (via metrics).

---

## Phase 3: Learning & Ecosystem (Gamma)
**Goal**: Total automation and community scalability.

### 3.1 Structural Auto-Discovery
- [ ] Integration of **Drain3** (or another log mining algorithm) in Rust.
- [ ] Template inference for unknown tools.

### 3.2 WASM Plugin System
- [ ] Support for complex filters written in WebAssembly.
- [ ] Total isolation of third-party plugins.

### 3.3 Universal Schema Hub
- [ ] Synchronization with a central schema repository.
- [ ] Sharing of anonymized structural templates.

### 3.4 Smart Aggregator (Future Enhancement)
- [ ] Implement the "Variable Buffer": Capture dynamic data (`<HEX>`, `<NUM>`) from collapsed patterns.
- [ ] Synthetic Summary Generation: Move from "5 lines collapsed" to "Items [0x1..0x5] processed ok".
- [ ] Plugin-ready architecture for tool-specific aggregators.

---

## Phase 4: Extreme Synthesis (Vision)
**Goal**: Total semantic inference and narrative summaries.
