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
- [x] **Process Detective**: Auto-prefixing only when a command is launched by an AI agent.
- [x] **Git Context**: Automatically prioritizing currently modified files.
- [ ] **Next**: "Local Shims" (Project-level binary overrides in `.axiom/bin`).

### 2.2 Semantic Transformer
- [x] Intent priority logic (Intent Overriding).
- [x] Basic keyword-based relevance matching.

### 2.3 Persistence & Analytics
- [x] SQLite integration for local history and template memory.

---

## Phase 3: Learning & Ecosystem (Gamma) - [IN PROGRESS]
**Goal**: Total automation and community scalability.

- [ ] **Developer Toolset**: Expanding default schemas for Linux (See [TOOLSET_ROADMAP.md](docs/TOOLSET_ROADMAP.md)).
- [x] Smart Aggregator: Variable Buffer and Synthetic Summary.

### 3.2 WASM Plugin System - [COMPLETED]
- [x] Support for complex filters written in WebAssembly.
- [x] Total isolation of third-party plugins.
- [x] **Plugin Developer Guide**: Comprehensive documentation for external logic.

### 3.3 Universal Schema Hub
- [ ] Synchronization with a central schema repository.
- [ ] Sharing of anonymized structural templates.

## Phase 3.5: Validation Lab (The Battle Arena) - [COMPLETED]
- [x] **Benchmark Suite**: Script to compare Raw vs Axiom output using real LLMs (Ollama/Groq).
- [x] **Token Metrics**: Automated calculation of token savings per command type.
- [x] **Instruction Feedback**: Verified `AGENTS.md` effectiveness with Gemini agent.

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

## 🛠️ Critical Review & Architectural Evolution (v0.1.0)

Basado en la auditoría técnica del proyecto, se han identificado los siguientes ejes de mejora crítica que deben priorizarse para garantizar la utilidad y el rendimiento de Axiom.

### 1. Optimización del Pipeline de Inteligencia (Performance)
*   **Problema**: El motor `NeuralIntelligence` (BERT) calcula embeddings en cada línea, lo cual es inviable en CPU para logs extensos.
*   **Acción**: 
    - [ ] Implementar **Caching de Intent Embeddings**: Calcular el embedding del intent una sola vez por sesión.
    - [ ] **Estrategia Híbrida Agresiva**: Usar el motor neural solo como "árbitro" final cuando los motores de reglas y palabras clave tengan baja confianza.
    - [ ] Explorar modelos más ligeros (FastText o SLMs especializados) para clasificación de relevancia en tiempo real.

### 2. Integración con la Terminal (Fidelidad)
*   **Problema**: El uso de Pipes (`Stdio::piped()`) rompe la interactividad y la estética (colores/spinners) de las herramientas CLI originales.
*   **Acción**:
    - [ ] **Migración a PTY (Pseudo-Terminales)**: Implementar la captura de salida usando PTYs para preservar el soporte de TTY, colores ANSI y prompts interactivos.

### 3. Refinamiento de la Privacidad (Falsos Positivos)
*   **Problema**: La redacción por entropía (umbral 4.5) genera demasiados falsos positivos en datos técnicos legítimos (Hashes de Git, IDs de Docker).
*   **Acción**:
    - [ ] **Context-Aware Redaction**: Implementar una lista blanca de patrones técnicos conocidos (SHA-1, SHA-256, SemVer).
    - [ ] Ajustar dinámicamente el umbral de entropía según el tipo de comando detectado por el `ToolSchema`.

### 4. Robustez de la IA Local (Resiliencia)
*   **Problema**: La descarga en caliente de modelos desde HuggingFace (`hf_hub`) rompe la promesa de "Local-First" en entornos sin internet.
*   **Acción**:
    - [ ] Implementar un comando `axiom setup` o `axiom pull-models` para pre-cargar los modelos necesarios.
    - [ ] Añadir mecanismos de *Graceful Degradation* al motor Fuzzy si el motor Neural no está disponible.

---
*"From raw bytes to semantic intent."*
