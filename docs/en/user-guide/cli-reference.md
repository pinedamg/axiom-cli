# CLI Reference

The Axiom CLI provides several commands to manage your installation, view analytics, and debug configurations.

## Global Flags

- **`--markdown`**: Enables Markdown table transformation globally.

## Core Commands

### `axiom <command>`
The primary usage. Acts as a proxy for the provided command.
- **Usage**: `axiom npm install`, `axiom docker logs my-container`
- **Behavior**: Intercepts the command's output, applies privacy filters, semantic compression, and outputs the optimized stream.

### `axiom install`
Installs Axiom shell integration.
- **Usage**: `axiom install`
- **Behavior**: Outputs commands to enable Axiom automatically for common commands in your shell configuration.

### `axiom gain`
Displays analytics on your token and cost savings.
- **Usage**: `axiom gain`
- **Flags**:
  - `--history`, `-s`: Shows a detailed list of recent command executions and the exact token savings for each.

### `axiom discovery`
Lists currently learned structural templates.
- **Usage**: `axiom discovery`

### `axiom check-ai`
Checks if the current process was called by an AI agent.
- **Usage**: `axiom check-ai`
- **Behavior**: Returns exit code 0 if called by an AI agent, 1 if called by a human shell.

## Configuration Commands

### `axiom config init`
Initializes a local `.axiom.yaml` file with default values.
- **Usage**: `axiom config init`
