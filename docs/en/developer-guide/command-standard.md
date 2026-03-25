# Axiom Command Enhancement Protocol (ACEP)

This document defines the industrial standard for enhancing base CLI commands within the Axiom ecosystem. Every command added or improved must adhere to the "Three-Layer Architecture" to ensure consistency, performance, and token efficiency.

## 1. The Three-Layer Architecture

When enhancing a command, implement these three versions of its output:

*   **V1: Structural Synthesis (Performance):** 
    *   Identify repetitive patterns in the raw output.
    *   Use `DiscoveryEngine` to group similar items (e.g., "15 files by extension", "5 worker processes").
    *   *Goal:* Reduce line count by at least 70%.

*   **V2: Semantic Insight (Intelligence):** 
    *   Inject high-level context using `AxiomEngine::generate_semantic_insight()`.
    *   Detect project markers (e.g., `Cargo.toml`, `package.json`, `.git`).
    *   *Goal:* Answer "What is this?" and "What should I do next?" proactively.

*   **V3: Privacy Redaction (Security):** 
    *   Identify sensitive data (hidden files, environment variables, keys, PII).
    *   Set default rules in the schema to `redact` or `hidden` this information.
    *   *Goal:* Safe output for AI consumption and shared environments.

---

## 2. Step-by-Step Workflow

### Step 1: Output Analysis (The "Raw" Phase)
Run the command in various environments and capture its typical output.
```bash
# Example: Analysis of 'git status'
git status --porcelain
```
Identify what is **Constant** (Structure) and what is **Variable** (Noise).

### Step 2: Schema Definition
Create or update the YAML schema in `config/schemas/<command>.yaml`.
*   Assign `priority` to rules (higher for specific patterns).
*   Use `synthesize` for groups of data.
*   Use `collapse` for known low-signal lines (headers, totals).
*   Use `redact` for sensitive patterns.

### Step 3: Engine Integration
If the command requires custom parsing (like `ls` column splitting):
1.  Update `src/engine/discovery.rs` with specific `parse_<command>_line` logic.
2.  Ensure `synthesize_line` handles the new format.
3.  **Critical:** Abstract variables using `<NUM>`, `<MONTH>`, `<TIME>`, and `<VAR>` to avoid template explosion in the DB.

### Step 4: Semantic Injection
Update `AxiomEngine::generate_semantic_insight()` in `src/engine/mod.rs`:
1.  Identify specific markers for the command.
2.  Return a human-readable, token-efficient string starting with "Detected...".

### Step 5: SNR & Token Optimization
*   **Header:** Ensure the command output uses the compact `[AXIOM]` header in the gateway.
*   **Prefixes:** Never repeat `[AXIOM]` per line. Use the bullet point `•` for items.
*   **Flush:** Only flush summaries at the end of the command execution to prevent output fragmentation.

### Step 6: Validation (The "Golden Path")
1.  **Live Test:** Run `axiom <command>` and verify the output.
2.  **Fixture:** Save the output to `tests/fixtures/<command>_axiom.txt`.
3.  **Regression:** Create/Update `tests/<command>_versions_test.rs` to verify that future changes don't break the synthesis or insights.

---

## 3. Best Practices
*   **Fail Safe:** If the engine can't parse a line, default to `keep` (Raw) or generic `Discovery` (Noise detection).
*   **Token Consciousness:** Use short, descriptive insights. Avoid conversational filler.
*   **Privacy First:** When in doubt, redact. The user can always request "unredacted" via intent if necessary.
