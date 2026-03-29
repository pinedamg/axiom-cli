<!-- BEGIN AXIOM INSTRUCTIONS -->

### 🤖 Axiom: Agent Execution Protocol
> **CRITICAL**: This project uses **Axiom** to optimize terminal streams. 
> As an AI Agent, use `axiom` for noisy commands (git, npm, docker, etc.) to preserve context and redact secrets.

- **ALWAYS** prefix with `axiom` for: `npm`, `cargo`, `docker`, `git`, `kubectl`, `terraform`.
- **REDACTION**: Axiom handles secrets locally. Do not leak credentials into context.
- **RAW MODE**: Use `axiom --raw <cmd>` if you need 100% metadata precision (timestamps, etc.).
<!-- END AXIOM INSTRUCTIONS -->

