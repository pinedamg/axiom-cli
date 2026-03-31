# Core Concepts

Axiom is built on two primary concepts: **Semantic Compression** and **Local Privacy**.

## Semantic Compression

Current AI agents are "context-hungry." When you run a command, 90% of the output is repetitive noise that:
1.  **Drains your wallet:** You pay for thousands of unnecessary tokens.
2.  **Loses context:** Critical errors get buried under thousands of lines of successful logs.

Axiom fixes this by acting as a **Semantic Firewall**.

- **Smart Aggregation**: Axiom doesn't just cut off logs. It understands structure. It compresses 100+ lines of success into a single dense summary.
  *Example*: Instead of 100 lines saying "Downloading package X", you get `[AXIOM] 124 packages downloaded successfully. IDs: [X...Y]`.
- **Intent Overriding**: Axiom detects if a command is failing. If you are debugging a specific error, Axiom force-shows relevant logs while suppressing the rest, ensuring the AI agent only sees the signal, not the noise.

## Local Privacy Shield

Axiom ensures sensitive data never leaves your machine.

- **Entropy Scanning**: Axiom automatically detects and redacts high-entropy strings (such as API keys, cloud secrets, and authentication tokens) using Shannon Entropy metrics.
- **PII Redaction**: A built-in engine masks emails, IPs, and sensitive patterns before they ever reach the AI agent's context window.

This all happens locally on your machine with sub-10ms overhead. No raw logs are ever sent to Axiom's servers.

## Intelligence Depth Levels

Axiom provides four levels of filtering depth to balance between token savings and metadata precision.

| Level | Command / Flag | Processing | Best for... |
| :--- | :--- | :--- | :--- |
| **RAW** | `axiom -r <cmd>` | None (Bypass) | Precision debugging, forensics, or timestamps. |
| **OFF** | `axiom intent disable` | Structure + Privacy | Routine tasks where you want summaries but no AI filtering. |
| **FUZZY** | `axiom intent enable fuzzy` | Keywords + Git Context | Standard development. Filters noise based on relevance. |
| **NEURAL** | `axiom intent enable neural` | Semantic Embeddings | Complex debugging where meaning matters more than keywords. |