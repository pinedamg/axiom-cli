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

### `axiom check-ai`
Checks if the current process was called by an AI agent (e.g., Cursor, Claude Code) or a human shell.
- **Usage**: `axiom check-ai`

### `axiom intent status`
Shows the current health, configuration, intent status, and telemetry status of your Axiom installation.
- **Usage**: `axiom intent status`
- **Output**: Session ID, Intelligence Mode, Parent Process, and Last Intent.

### `axiom intent enable <mode>`
Enables intent intelligence to filter output based on user/agent intent.
- **Modes**: `fuzzy` (keywords) or `neural` (AI embeddings).
- **Usage**: `axiom intent enable neural`

### `axiom intent disable`
Disables intent intelligence. Axiom will maintain formatting but show all files without relevance filtering.
- **Usage**: `axiom intent disable`

### `axiom proxy <cmd>`
Executes the raw command without filtering. Useful for debugging or bypassing Axiom entirely for a specific execution.
- **Usage**: `axiom proxy npm install`

### `axiom discovery`
Lists currently learned structural templates and their frequency of use in the session.
- **Usage**: `axiom discovery`

## Configuration Commands

### `axiom config init`
Initializes a local `.axiom.yaml` configuration file with default values.
- **Usage**: `axiom config init`

## Global Flags

### `-m, --markdown`
Enables automatic transformation of space-aligned table outputs into real Markdown tables.
- **Usage**: `axiom -m kubectl get pods`
