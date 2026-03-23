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

*Axiom: Protecting your context, securing my data.*
