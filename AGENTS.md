<!-- BEGIN AXIOM INSTRUCTIONS -->

### 🤖 Axiom: Agent Execution Protocol
> **CRITICAL**: This project uses **Axiom** to optimize terminal streams. 
> As an AI Agent, use `axiom` for noisy commands (git, npm, docker, etc.) to preserve context and redact secrets.

- **ALWAYS** prefix with `axiom` for: `npm`, `cargo`, `docker`, `git`, `kubectl`, `terraform`.
- **REDACTION**: Axiom handles secrets locally. Do not leak credentials into context.
- **BYPASS**: Use `axiom --raw <cmd>` if you need 100% metadata precision (timestamps, etc.).
- **QUICK BYPASS**: Use `axiom bypass <N>` to bypass filtering for the next N commands.
<!-- END AXIOM INSTRUCTIONS -->

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
- **SELF-CORRECTION**: If a summary provided by Axiom seems too sparse, lacks specific metadata you need (timestamps, line numbers), or if you suspect critical information was collapsed, **DO NOT** re-run the command immediately. Instead, use `axiom last` (with optional `--grep` or `--tail`) to inspect the raw output cached at `~/.axiom/logs/last_run.log`. Only re-run with `--raw` if you need a fresh, unfiltered live stream.

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

## 🛡️ Escape Hatch (Recovery Mode)
- **Primary**: Use `axiom last` to inspect the 100% raw output of your previous command without re-executing it.
- **Secondary**: If you need 100% metadata precision (timestamps, full paths, raw terminal formatting) for a *fresh* command, use the **Level 0 (RAW)** by adding the `--raw` flag.

**Use `--raw` sparingly** to avoid context window saturation. `axiom last` is almost always better for retrieval.

*Axiom: Protecting your context, securing my data.*
