# CLI Reference

The Axiom CLI provides several commands to manage your installation, view analytics, and debug configurations.

## Core Commands

### `axiom <command>`
The primary usage. Acts as a proxy for the provided command.
- **Usage**: `axiom npm install`, `axiom docker logs my-container`
- **Behavior**: Intercepts the command's output, applies privacy filters, semantic compression, and outputs the optimized stream.

### `axiom gain`
Displays analytics on your token and cost savings.
- **Usage**: `axiom gain`
- **Flags**:
  - `--history`: Shows a detailed list of recent command executions and the exact token savings for each.

### `axiom status`
Shows the current health, configuration, and telemetry status of your Axiom installation.
- **Usage**: `axiom status`
- **Output**: Edition (Community/Pro), Telemetry Level, Installation ID, and active schemas.

### `axiom check-ai`
Checks if the current process was called by an AI agent. It uses the `ProcessDetective` to determine if the parent process is a known AI agent (like Cursor, Claude Code, or Gemini CLI) or a human shell.
- **Usage**: `axiom check-ai`

### `axiom discovery`
Lists all currently learned structural templates that the Discovery Engine has synthesized from past command executions.
- **Usage**: `axiom discovery`

### `axiom proxy <cmd>`
Executes the raw command without filtering. Useful for debugging or bypassing Axiom entirely for a specific execution.
- **Usage**: `axiom proxy npm install`

### `axiom discover`
*(Beta)* Analyzes local AI agent history (like Claude Code) to find missed opportunities where Axiom could have saved tokens.
- **Usage**: `axiom discover`

## Intent Management

### `axiom intent enable <mode>`
Enables intent intelligence. Mode can be either `fuzzy` (based on keywords) or `neural` (using local AI embeddings).
- **Usage**: `axiom intent enable fuzzy` or `axiom intent enable neural`

### `axiom intent disable`
Disables intent intelligence. Axiom will maintain formatting and privacy redaction but will not hide files or outputs based on relevance.
- **Usage**: `axiom intent disable`

### `axiom intent status`
Shows the current intent discovery status, the intelligence mode, the session ID, and the relevant files/intent detected from the parent process.
- **Usage**: `axiom intent status`

## Configuration Commands

### `axiom config init`
Initializes a local `.axiom.yaml` configuration file in the current directory with default values.
- **Usage**: `axiom config init`

### `axiom config telemetry <level>`
Sets your preferred telemetry level.
- **Levels**: `full`, `discovery`, `anonymous`, `off` (Pro only).
- **Example**: `axiom config telemetry discovery`

### `axiom config license <key>`
Applies a Pro license key to unlock premium features like Offline telemetry mode.
- **Example**: `axiom config license abc-123-xyz`

## Global Flags

### `--markdown` (`-m`)
Enables Markdown table transformation. When this flag is passed, Axiom attempts to detect tabular outputs (like from `ps` or `docker ps`) and transforms them into standard Markdown tables for better LLM readability.
- **Usage**: `axiom --markdown <command>`
