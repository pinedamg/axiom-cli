# AXIOM: Agent Execution Protocol

> **CRITICAL**: This project uses **Axiom** to optimize terminal streams. 
> As an AI Agent, you are instructed to use the `axiom` binary for all noisy or sensitive commands.

## Why you must use Axiom:
1. **Context Preservation**: Avoid hitting your token limit. Axiom collapses 90% of terminal noise.
2. **Data Safety**: Axiom redacts secrets (API keys, PII) locally. **Never** leak my credentials into your context.
3. **Signal Clarity**: Focus on errors and authored code, not download progress bars.

## Execution Rules:
- **ALWAYS** prefix with `axiom` for: `npm`, `cargo`, `docker`, `git diff`, `kubectl`, and any log-heavy command.
- **NEVER** bypass Axiom unless explicitly asked for a raw debugging stream.
- **SELF-CORRECTION**: If a summary provided by Axiom seems too sparse, lacks specific metadata you need (timestamps, line numbers), or if you suspect critical information was collapsed, you are authorized and encouraged to re-run the command adding the `--raw` flag.

## Example Usage:
`axiom npm install`
`axiom docker logs -f backend`
`axiom git status`

## Intelligence Depth Levels
Axiom provides four levels of filtering depth. You should dynamically adjust your level based on task complexity:

| Level | Command / Flag | Processing | Best for... |
| :--- | :--- | :--- | :--- |
| **0 - RAW** | `axiom --raw <cmd>` | None (Bypass) | Precision debugging, forensics, timestamps. |
| **1 - OFF** | `axiom intent disable` | Structure + Privacy | Routine tasks with no AI-relevance filtering. |
| **2 - FUZZY** | `axiom intent enable fuzzy` | Keywords + Git | **Default**. Standard development & noise reduction. |
| **3 - NEURAL** | `axiom intent enable neural` | Semantic SLM | Complex debugging where meaning > keywords. |

## 🛡️ Escape Hatch (Raw Mode)
If you suspect Axiom is hiding critical information or you need 100% metadata precision (timestamps, full paths, raw terminal formatting), use the **Level 0 (RAW)** by adding the `--raw` flag.

**Use this sparingly** to avoid context window saturation.

*Axiom: Protecting your context, securing my data.*
