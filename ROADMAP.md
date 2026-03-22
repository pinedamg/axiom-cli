# AXIOM: Development Roadmap

This Roadmap defines the implementation path for **AXIOM**, prioritizing security and minimal latency.

## Phase 1: Foundation & Security (Alpha) - [COMPLETED]
**Goal**: Build the flow interceptor and the data protection system.
- [x] CLI proxy, non-blocking capture, entropy scanner, PII redaction, and YAML engine.

## Phase 2: Intelligence & Context (Beta) - [IN PROGRESS]
**Goal**: Make Axiom understand the "why" behind command execution.

### 2.1 Auto-Intent Detection
- [x] Manual context via environment variables.
- [ ] **Next**: Auto-discovery of chat logs (Cursor, Claude, Gemini CLI) to extract context silently.
- [ ] Git context integration (detecting currently modified files).

### 2.2 Semantic Transformer
- [x] Intent priority logic (Intent Overriding).
- [x] Basic keyword-based relevance matching.

### 2.3 Persistence & Analytics
- [x] SQLite integration for local history and template memory.

---

## Phase 3: Learning & Ecosystem (Gamma) - [IN PROGRESS]
**Goal**: Total automation and community scalability.

### 3.1 Structural Auto-Discovery
- [x] Pattern-based template extraction and structural skeletons.

### 3.2 WASM Plugin System
- [ ] Support for complex filters written in WebAssembly.
- [ ] Total isolation of third-party plugins.

### 3.3 Universal Schema Hub
- [ ] Synchronization with a central schema repository.
- [ ] Sharing of anonymized structural templates.

### 3.4 Smart Aggregator
- [x] Variable Buffer: Capture dynamic data (`<HEX>`, `<NUM>`, `<UUID>`, `<PATH>`).
- [x] Synthetic Summary Generation: "Template matched X times. Variables: [...]".

---

## Phase 4: Local AI & Semantic Intelligence (Vision)
**Goal**: Move beyond keywords into true meaning.

### 4.1 Local Embeddings (SLM)
- [ ] Integration of ONNX Runtime or Candle for local vector similarity.
- [ ] Replace keyword matching with **Semantic Similarity** (>0.8 score = relevant).

### 4.2 Neural Aggregator
- [ ] Use a Small Language Model to narrate summaries of repetitive logs.
- [ ] Anomaly detection: Highlight logs that deviate from the structural norm.

---
*"From raw bytes to semantic intent."*
