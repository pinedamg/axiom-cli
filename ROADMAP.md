# AXIOM: Development Roadmap

This Roadmap defines the implementation path for **AXIOM**, prioritizing security and minimal latency.

## Phase 1: Foundation & Security (Alpha) - [COMPLETED]
**Goal**: Build the flow interceptor and the data protection system.
- [x] CLI proxy, non-blocking capture, entropy scanner, PII redaction, and YAML engine.
- [x] **Performance Optimization**: Achieved <10ms startup latency (~9ms in Release) via SQLite tuning.

## Phase 2: Intelligence & Context (Beta) - [IN PROGRESS]
**Goal**: Make Axiom understand the "why" behind command execution.

### 2.1 Auto-Intent & Integration
- [x] Manual context via environment variables.
- [x] Auto-discovery of chat logs (Cursor, Claude, Gemini CLI) to extract context silently.
- [ ] **Next**: "Process Detective" (Auto-prefixing only when a command is launched by an AI agent).
- [ ] **Next**: "Local Shims" (Project-level binary overrides in `.axiom/bin`).
- [ ] Git context integration (detecting currently modified files).

### 2.2 Semantic Transformer
- [x] Intent priority logic (Intent Overriding).
- [x] Basic keyword-based relevance matching.

### 2.3 Persistence & Analytics
- [x] SQLite integration for local history and template memory.

---

## Phase 3: Learning & Ecosystem (Gamma) - [IN PROGRESS]
[...]
- [x] Smart Aggregator: Variable Buffer and Synthetic Summary.

## Phase 3.5: Validation Lab (The Battle Arena) - [COMPLETED]
- [x] **Benchmark Suite**: Script to compare Raw vs Axiom output using real LLMs (Ollama/Groq).
- [x] **Token Metrics**: Automated calculation of token savings per command type.
- [ ] **Instruction Feedback**: Test `AGENTS.md` effectiveness with automated agents.

## Phase 4: Local AI & Semantic Intelligence (Vision)
[...]

## Phase 4: Local AI & Semantic Intelligence (Vision) - [IN PROGRESS]
**Goal**: Move beyond keywords into true meaning.

### 4.1 Local Embeddings (SLM)
- [x] Integration of **Candle** (Pure Rust) for local vector similarity.
- [x] Replace/Augment keyword matching with **Semantic Similarity** (BERT-based).
- [x] Hybrid strategy: Keyword -> Fuzzy -> Neural.

### 4.2 Neural Aggregator
- [ ] Use a Small Language Model to narrate summaries of repetitive logs.
- [ ] Anomaly detection: Highlight logs that deviate from the structural norm.

---
*"From raw bytes to semantic intent."*
