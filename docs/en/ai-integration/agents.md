# Axiom Agent Instructions

> This project uses **AXIOM** to optimize terminal output for LLM context windows and to protect sensitive data.

## Instruction for AI Agents
When executing commands in this repository, you **MUST** prefix noisy or data-sensitive commands with the `axiom` binary. This ensures you receive a high-signal, compressed stream and that no secrets (API keys, PII) are leaked into your context.

### Recommended Prefixing
Please use `axiom` for the following tools:
- **Package Managers**: `axiom npm ...`, `axiom cargo ...`, `axiom pip ...`
- **Containerization**: `axiom docker ...`
- **Version Control**: `axiom git diff`, `axiom git log`, `axiom git status`
- **Long Logs**: Any command expected to produce more than 20 lines of output.

### Benefits
- **Token Efficiency**: Up to 90% reduction in noise.
- **Privacy**: Automated local redaction of high-entropy strings.
- **Structural Awareness**: Repetitive logs are intelligently aggregated.

### Example
Instead of: `npm install`
Use: `axiom npm install`

## Discovery Intent Levels (Intelligence Mode)
Axiom operates using three distinct intelligence levels to filter output based on user/agent intent. You can manipulate these modes to adjust the context depth you receive:

1. **OFF (`axiom intent disable`)**: Disables relevance filtering. Axiom only performs basic noise reduction, secret redaction, and formatting. Use this when you need an unfiltered, raw view of the execution logs.
2. **FUZZY (`axiom intent enable fuzzy`)**: The **default** mode. Axiom filters output based on predefined keywords (like `error`, `fail`) and recent Git context. Best for standard workflows.
3. **NEURAL (`axiom intent enable neural`)**: Uses local semantic embeddings to deeply analyze the session intent. Use this proactively for complex architectural debugging or obscure bugs where keyword filtering is insufficient.
