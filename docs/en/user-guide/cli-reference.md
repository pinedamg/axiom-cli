# CLI Reference

The Axiom CLI provides several commands to manage your installation, view analytics, and debug configurations.

## Core Commands

### `axiom <command>`
The primary usage. Acts as a proxy for the provided command.
- **Usage**: `axiom npm install`, `axiom docker logs my-container`
- **Behavior**: Intercepts the command's output, applies privacy filters, semantic compression, and outputs the optimized stream.

### `axiom install`
Installs the Axiom shell integration.
- **Usage**: `axiom install`
- **Behavior**: Provides instructions and aliases to add to your shell configuration to automatically route noisy commands through Axiom.

### `axiom gain`
Displays analytics on your token and cost savings.
- **Usage**: `axiom gain`
- **Flags**:
  - `-s, --history`: Shows a detailed list of recent command executions and the exact token savings for each.

### `axiom discovery`
Lists the currently learned structural templates.
- **Usage**: `axiom discovery`
- **Behavior**: Shows the structural patterns Axiom has learned from your commands and their frequencies.

### `axiom check-ai`
Checks if the current process was called by an AI agent.
- **Usage**: `axiom check-ai`
- **Behavior**: Detects if the shell running the command belongs to an AI agent (like Cursor or Claude Code) and exits with 0 if true, 1 if false.

### `axiom intent <action>`
Manages Intent Discovery and Intelligence Levels.
- **Actions**:
  - `enable [mode]`: Enables intent intelligence. `mode` can be `fuzzy` (keywords) or `neural` (AI embeddings). Defaults to `fuzzy`.
  - `disable`: Disables intent intelligence (maintains formatting but shows all files).
  - `status`: Shows current intent discovery status, including Session ID, Intelligence Mode, Parent Process, and Last Intent.

## Configuration Commands

### `axiom config init`
Initializes a local `.axiom.yaml` configuration file with default values.
- **Usage**: `axiom config init`
- **Behavior**: Creates a new `.axiom.yaml` in the current directory if one doesn't exist.
