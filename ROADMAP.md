# AXIOM: Development Roadmap

This Roadmap defines the implementation path for **AXIOM**, prioritizing security and minimal latency.

## Phase 1: Foundation & Security (Alpha)
**Goal**: Build the flow interceptor and the data protection system.

### 1.1 Gateway Interceptor
- [x] Implement the CLI proxy (`axiom <cmd>`).
- [x] Non-blocking capture of `stdout` and `stderr`.
- [x] Initial latency budget: <5ms overhead.

### 1.2 Privacy Layer (The Shield)
- [x] Implement **Shannon Entropy** scanner to detect secrets (API Keys, hashes).
- [x] Regex-based redaction engine for PII (Personally Identifiable Information).
- [x] "Blind-by-Design" mode: Local redaction before any semantic processing.

### 1.3 Static YAML Schemas
- [x] Basic YAML-based rule engine for common tools (`git`, `docker`, `npm`).
- [x] Support for fixed and variable text blocks.

---

## Phase 2: Intelligence & Context (Beta)
**Goal**: Make Axiom understand the "why" behind command execution.

### 2.1 Intent-Aware Engine
- [x] Integration with LLM chat history (via environment/manual context).
- [x] Extraction of key entities from the prompt (keyword matching).

### 2.2 Semantic Transformer
- [x] Dynamic collapse/expansion logic based on "Intent".
- [x] Flow prioritization logic implemented in Engine.

### 2.3 Persistence & Analytics
- [x] SQLite integration for local history.
- [x] Token saving telemetry and template memory.

---

## Phase 3: Learning & Ecosystem (Gamma)
**Goal**: Total automation and community scalability.

### 3.1 Structural Auto-Discovery
- [x] Pattern-based template extraction.
- [x] Template inference for unknown tools using structural skeleton.

### 3.2 WASM Plugin System
- [ ] Support for complex filters written in WebAssembly.
- [ ] Total isolation of third-party plugins.

### 3.3 Universal Schema Hub
- [ ] Synchronization with a central schema repository.
- [ ] Sharing of anonymized structural templates.

### 3.4 Smart Aggregator (Current Focus)
- [ ] Implement the "Variable Buffer": Capture dynamic data (`<HEX>`, `<NUM>`) from collapsed patterns.
- [ ] Synthetic Summary Generation: Move from "5 lines collapsed" to "Items [0x1..0x5] processed ok".
- [ ] Plugin-ready architecture for tool-specific aggregators.

---

## Phase 4: Extreme Synthesis (Vision)
**Goal**: Total semantic inference and narrative summaries.
