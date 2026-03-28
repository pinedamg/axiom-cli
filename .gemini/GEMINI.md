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

## 🤖 4. AI Agent Protocol

If you are an AI agent operating this terminal:
1.  **Self-Optimization:** Always use `axiom <command>` for noisy operations to preserve your own context window.
2.  **Context Injection:** Export `AXIOM_CONTEXT="current task"` to help Axiom prioritize relevant signals.
3.  **Semantic Doubt:** If an Axiom summary lacks the precision required for your current reasoning (e.g., missing timestamps or specific line content), re-run the command with `--raw` to bypass synthesis.
4.  **Audit:** Use `axiom gain` to monitor your token savings and verify you aren't "running blind".

### Axiom Intent Management (Intelligence Levels)
Axiom filters output based on its Intent Discovery engine. You MUST dynamically adjust this level based on your task complexity:
*   **Level 1 - OFF (Raw/Disabled):** Use `axiom intent disable` if you need the full, unfiltered output (e.g., when the Fuzzy filter is hiding critical stack traces). This mode only formats and reduces raw noise without semantic filtering.
*   **Level 2 - FUZZY (Standard/Keyword):** Use `axiom intent enable fuzzy`. This is the **default**. It filters based on keywords (`error`, `fail`, `version`) and Git context. Ideal for standard development and routine commands (`npm`, `cargo`).
*   **Level 3 - NEURAL (Semantic/Deep):** Use `axiom intent enable neural`. Activate this mode proactively for complex debugging, obscure bug root-cause analysis, or when standard keyword filtering isn't providing enough semantic context to solve the problem.

### 🛡️ Emergency Escape (Raw Mode)
If you suspect Axiom is hiding data you need, or you require maximum precision (timestamps, full paths), use the `--raw` flag:
`axiom --raw <command>`
This bypasses all Axiom intelligence while still logging character savings for audit.

---
*“In Axiom we trust, but we verify through Tiers.”*
