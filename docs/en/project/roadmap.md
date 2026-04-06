# AXIOM: Development & Toolset Roadmap

This combined Roadmap defines the implementation path for **AXIOM**, prioritizing security, minimal latency, and the expansion of intelligent schemas.

---

## 🏗️ Core Development Roadmap

### Phase 1: Foundation & Security (Alpha) - [COMPLETED]
**Goal**: Build the flow interceptor and the data protection system.
- [x] CLI proxy, non-blocking capture, entropy scanner, PII redaction, and YAML engine.
- [x] **Performance Optimization**: Achieved <10ms startup latency (~9ms in Release) via SQLite tuning.

### Phase 2: Intelligence & Context (Beta) - [IN PROGRESS]
**Goal**: Make Axiom understand the "why" behind command execution.
#### 2.1 Auto-Intent & Integration
- [x] Manual context via environment variables.
- [x] Auto-discovery of chat logs (Cursor, Claude, Gemini CLI) to extract context silently.
- [x] **Process Detective**: Auto-prefixing only when a command is launched by an AI agent.
- [x] **Git Context**: Automatically prioritizing currently modified files.
- [ ] **Next**: "Local Shims" (Project-level binary overrides in `.axiom/bin`).

#### 2.2 Semantic Transformer
- [x] Intent priority logic (Intent Overriding).
- [x] Basic keyword-based relevance matching.

#### 2.3 Persistence & Analytics
- [x] SQLite integration for local history and template memory.

### Phase 3: Learning & Ecosystem (Gamma) - [IN PROGRESS]
**Goal**: Total automation and community scalability.
- [ ] **Developer Toolset**: Expanding default schemas for Linux (See Toolset section below).
- [x] Smart Aggregator: Variable Buffer and Synthetic Summary.

#### 3.2 WASM Plugin System - [COMPLETED]
- [x] Support for complex filters written in WebAssembly.
- [x] Total isolation of third-party plugins.
- [x] **Plugin Developer Guide**: Comprehensive documentation for external logic.

#### 3.3 Universal Schema Hub
- [ ] Synchronization with a central schema repository.
- [ ] Sharing of anonymized structural templates.

### Phase 3.5: Validation Lab (The Battle Arena) - [COMPLETED]
- [x] **Benchmark Suite**: Script to compare Raw vs Axiom output using real LLMs (Ollama/Groq).
- [x] **Token Metrics**: Automated calculation of token savings per command type.
- [x] **Instruction Feedback**: Verified `AGENTS.md` effectiveness with Gemini agent.

### Phase 4: Local AI & Semantic Intelligence (Vision) - [IN PROGRESS]
**Goal**: Move beyond keywords into true meaning.

#### 4.1 Local Embeddings (SLM)
- [x] Integration of **Candle** (Pure Rust) for local vector similarity.
- [x] Replace/Augment keyword matching with **Semantic Similarity** (BERT-based).
- [x] Hybrid strategy: Keyword -> Fuzzy -> Neural.

#### 4.2 Neural Aggregator
- [ ] Use a Small Language Model to narrate summaries of repetitive logs.
- [ ] Anomaly detection: Highlight logs that deviate from the structural norm.

---

## 🛠️ Critical Review & Architectural Evolution (v0.1.0)

Basado en la auditoría técnica del proyecto, se han identificado los siguientes ejes de mejora crítica:

### 1. Optimización del Pipeline de Inteligencia (Performance)
*   **Problema**: El motor `NeuralIntelligence` (BERT) calcula embeddings en cada línea, inviable en CPU.
*   **Acción**: 
    - [ ] Implementar **Caching de Intent Embeddings**: Calcular una sola vez por sesión.
    - [ ] **Estrategia Híbrida Agresiva**: Neural como "árbitro" final.
    - [ ] Explorar modelos más ligeros (FastText/SLMs).

### 2. Integración con la Terminal (Fidelidad)
*   **Problema**: Uso de Pipes rompe la interactividad y colores.
*   **Acción**:
    - [ ] **Migración a PTY (Pseudo-Terminales)**.

### 3. Refinamiento de la Privacidad (Falsos Positivos)
*   **Problema**: Entropía genera falsos positivos (Hashes, IDs).
*   **Acción**:
    - [ ] **Context-Aware Redaction**: Lista blanca de patrones (SHA, SemVer).
    - [ ] Ajustar dinámicamente umbrales según `ToolSchema`.

### 4. Robustez de la IA Local (Resiliencia)
*   **Problema**: Descarga en caliente rompe promesa Local-First.
*   **Acción**:
    - [ ] Comando `axiom setup` para pre-cargar modelos.
    - [ ] Mecanismos de *Graceful Degradation*.

---

## 💎 Phase 5: Advanced Token Economics (RTK-Inspired)
**Goal**: Maximize the Return on Investment (ROI) of every token and automate system evolution.

### 5.1 Token ROI & Prediction Engine
- [ ] **Axiom Gain**: Advanced analytics dashboard showing cumulative savings in USD/Tokens.
- [ ] **Predictive Warning**: Alert agents when a command (e.g., `cat` on a huge file) will exceed a "Token Budget".
- [ ] **Economic Arbitration**: Suggest cheaper alternatives (e.g., `grep` vs `cat | grep`) before execution.

### 5.2 Autonomous Learning Loop (`axiom learn`)
- [ ] **Pattern Discovery**: Analyze shell history to identify "high-noise" commands without schemas.
- [ ] **Schema Auto-Generation**: Use LLM to suggest YAML schemas based on captured noisy outputs.
- [ ] **Error Correction**: Learn from "Agent Retries" (e.g., if an agent runs `ls` then `ls -a`, Axiom should adjust the default `ls` schema for that context).

### 5.3 Deep Structural Synthesis
- [ ] **Schema-Only Mode**: Transform massive JSON/YAML objects into "Shape Summaries" (keys and types only).
- [ ] **Semantic Diff**: Ultra-condensed diffs that prioritize logic changes over whitespace or trivial updates.
- [ ] **Universal Minifier**: A "lossy" compression mode for logs that preserves semantic meaning while destroying 90% of the characters.

---

## 📡 Phase 6: Telemetry & Control Plane (Axiom Pulse) - [IN PROGRESS]
**Goal**: Secure observability and user-centric value metrics.
- [x] **Hardware Handshake**: Unique identity anchored to PC hardware (SHA-256).
- [x] **Proof of Work (PoW)**: Anti-spam filter for node registrations.
- [x] **Cryptographic Signing (HMAC)**: Usage reports signed with unique node secrets.
- [x] **Axiom Pulse API**: High-performance ingestion with Redis validation.
- [ ] **Multi-machine Linking**: Connect multiple devices to a single user profile (Roadmap Idea 2).
- [ ] **Private Insights Dashboard**: Personalized view of accumulated token savings.

---

## 🧰 Developer Toolset Expansion Roadmap

This section defines the expansion of default schemas and intelligent modes for Linux developers.

### 🟢 Tier 1: The Core Fundamentals (High Frequency)
*Goal: Remove structural noise from everyday commands.*
- [x] **ls / tree**: Collapse hidden files, metadata, and junk directories.
- [x] **cat / tail / head**: "Guardian Mode" for files > 50 lines (auto-summary).
- [ ] **grep / rg (ripgrep)**: Aggregate matches per file and provide density summaries.
- [x] **curl / wget**: Hide progress bars and redundant HTTP headers.

### 🟡 Tier 2: Build & Dev Ecosytems (Context-Aware)
*Goal: Filter successful boilerplate and focus on warnings/errors.*
- [x] **npm / pnpm / yarn**: Basic installer noise reduction.
- [x] **cargo (Rust)**: Collapse dependency downloading/compiling. Force-show local crate warnings.
- [x] **go build / test**: Summarize test results.
- [ ] **pip / poetry / conda**: Clean virtualenv setup and logs.

### 🟠 Tier 3: Infrastructure & Cloud (Volume Control)
*Goal: Prevent context window saturation from massive infrastructure outputs.*
- [x] **docker / docker-compose**: Collapse layer pull progress and health-check loops.
- [x] **kubectl**: Summarize pod states, clean resource descriptions.
- [x] **terraform**: Synthesize `terraform plan`.
- [x] **aws / gcloud / az**: Transform JSON/Table listings into dense summaries.

### 🔵 Tier 4: Data & System (Structural Synthesis)
*Goal: Maintain data shape while reducing token count.*
- [x] **jq / yq**: Identify JSON structure and summarize arrays.
- [x] **ps / journalctl**: Deep cleaning of system/kernel noise.
- [x] **netstat / lsof / ss**: Filter system-reserved ports.

### 🚀 Advanced Intelligent Modes (Behavioral Flags)
- [x] **`--markdown`**: Automatically transform table outputs into real Markdown tables.
- **`--diff-only`**: Show only what changed since the last execution.
- **`--explain`**: Prepend a natural language summary of what Axiom compressed.
