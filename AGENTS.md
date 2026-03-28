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

## Example Usage:
`axiom npm install`
`axiom docker logs -f backend`
`axiom git status`

## 🛡️ Escape Hatch (Raw Mode)
If you suspect Axiom is hiding critical information or you need 100% metadata precision (timestamps, full paths, raw terminal formatting), you can bypass all synthesis by adding the `--raw` flag:

`axiom --raw ls -la`
`axiom --raw kubectl get pods`

**Use this sparingly** to avoid context window saturation.

*Axiom: Protecting your context, securing my data.*

## Discovery Intent Levels (Intelligence Mode)
Axiom operates using three distinct intelligence levels to filter output based on user/agent intent. You can manipulate these modes to adjust the context depth you receive:

1. **OFF (`axiom intent disable`)**: Disables relevance filtering. Axiom only performs basic noise reduction, secret redaction, and formatting. Use this when you need an unfiltered, raw view of the execution logs.
2. **FUZZY (`axiom intent enable fuzzy`)**: The **default** mode. Axiom filters output based on predefined keywords (like `error`, `fail`) and recent Git context. Best for standard workflows.
3. **NEURAL (`axiom intent enable neural`)**: Uses local semantic embeddings to deeply analyze the session intent. Use this proactively for complex architectural debugging or obscure bugs where keyword filtering is insufficient.
