# AXIOM: The Semantic Token Streamer 🦀

> "Signal over Noise. Protect secrets, save context, empower AI."

This document defines the architectural soul and the rigorous verification standards for Axiom. Every feature must be validated for **Token Reduction** and **Semantic Legibility**.

---

## 🎯 1. Mission & Core Philosophy

Axiom is a **Semantic Firewall**. It sits between raw CLI output and the AI context window to:
1.  **Redact Secrets:** Local entropy-based masking (Privacy First).
2.  **Compress Tokens:** Collapse 90% of noise (Token Economy).
3.  **Preserve Signal:** Ensure the "Intent" remains clear (Legibility).

---

## 🏗️ 2. Development Standards

*   **Core:** Pure Rust (Edition 2021). Performance overhead must be < 10ms.
*   **Engine:** Line-by-line streaming using `tokio` and regex-based schema matching.
*   **Intelligence:** Hybrid approach (Fuzzy Matcher + local Embeddings via `candle`).
*   **Extensibility:** WASM-based plugins for custom tool schemas.

---

## 🧪 3. The "Triple-Threat" Testing Strategy

We verify Axiom in three progressive phases to balance speed, accuracy, and quality.

### PHASE 1: Manual Validation (`axiom pipe`) - *On-Demand*
The primary way to test new logic "in the wild".
*   **Workflow:** Pipe any noisy command to Axiom to see the live results.
*   **Example:** `ls -R | axiom` or `docker logs -f my_container | axiom`.
*   **Goal:** Quick feedback on filtering. Does it feel "clean"? Does it show the important stuff?

### PHASE 2: Quantitative Validation (Snapshots & TCR) - *Automated*
Once a filter is refined, we "freeze" it to prevent regressions.
*   **Token Compression Ratio (TCR):** We measure `(Raw Tokens - Axiom Tokens) / Raw Tokens`.
*   **Snapshots:** We store `fixtures/ls_raw.txt` and compare output against `fixtures/ls_golden.txt`.
*   **Pass Criteria:** TCR must be > 70% for high-noise commands without losing "Golden Lines".

### PHASE 3: Qualitative Validation (AI Judge & SPS) - *Semantic*
The final boss of testing. Since Axiom is for IAs, we let an IA judge its quality.
*   **Semantic Preservation Score (SPS):** An external LLM (GPT-4/Gemini) compares the raw vs. compressed output.
*   **The Test:** Ask the Judge IA: *"Based on this compressed log, what was the specific error?"*.
*   **Pass Criteria:** If the Judge IA can answer correctly, SPS is 10/10. If it's confused, we are compressing too much.

---

## 🤖 4. AI Agent Protocol (Orchestration Manifesto)

Como agente operando en este repositorio, debes seguir el **Axiom Orchestration Manifesto** (ver `/GEMINI.md` en la raíz).

### Reglas Críticas:
1.  **Anti-Recursividad**: Durante el desarrollo, builds y tests de `axiom-cli`, **NO** usar el comando `axiom`. Usa comandos nativos (`cargo`, `npm`) para asegurar transparencia total.
2.  **Evidencia Obligatoria**: No declares una tarea como terminada sin incluir los logs de tests (`cargo test`) que validen el cambio.
3.  **Matriz de Impacto**: Antes de aplicar cambios en el CLI, evalúa si impactan en **Pulse** (Handshake) o **Web** (Docs).
4.  **Resiliencia y Recuperación (Shadow Logs)**: Si un resumen de Axiom oculta detalles necesarios, **NO** re-ejecutes el comando con `--raw` inmediatamente. Axiom guarda el 100% de la salida cruda en `~/.axiom/logs/last_run.log`. Usa `axiom last` (con `--grep` o `--tail`) para interrogar el log crudo sin quemar tokens innecesarios. Usa `--raw` solo si necesitas precisión absoluta en un flujo en vivo (timestamps, metadata de terminal, etc.).
5.  **Token Economy**: Tu prioridad es minimizar la basura en el context window. Si Axiom está activo, confía en su filtrado y solo "interroga" lo crudo si detectas una inconsistencia crítica.

---
*“In Axiom we trust, but we verify through the Manifesto.”*

