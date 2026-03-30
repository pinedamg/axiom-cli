# CLI Reference

The Axiom CLI provides several commands to manage your installation, view analytics, and debug configurations.

## Core Commands

### `axiom <command>`
The primary usage. Acts as a proxy for the provided command.
- **Usage**: `axiom npm install`, `axiom docker logs my-container`
- **Behavior**: Intercepts the command's output, applies privacy filters, semantic compression, and outputs the optimized stream.
- **Flags**:
  - `--raw`: Bypasses all Axiom processing and synthesis. Outputs the exact stream from the child process.
  - `--markdown`: Enables automatic transformation of terminal tables into Markdown format.
  - `--yes`: Automatically answer yes to all prompts.

### `axiom install`
Installs Axiom shell integration and AI context.
- **Usage**: `axiom install [OPTIONS]`
- **Flags**:
  - `--path <PATH>`: Project path to sync AI context (default: current dir).
  - `--context-only`: Only install AI context, skip shell aliases.

### `axiom uninstall`
Removes all Axiom traces from the system.
- **Usage**: `axiom uninstall [OPTIONS]`
- **Flags**:
  - `--path <PATH>`: Project path to remove AI context (default: current dir).

### `axiom doctor`
Runs system health check and diagnostics.
- **Usage**: `axiom doctor [OPTIONS]`
- **Flags**:
  - `--path <PATH>`: Project path to check AI context (default: current dir).
  - `--fix`: Attempt to automatically fix detected issues.

### `axiom self-update`
Updates Axiom to the latest version from GitHub.
- **Usage**: `axiom self-update`

### `axiom last`
Shows the raw output of the last executed command.
- **Usage**: `axiom last [OPTIONS]`
- **Flags**:
  - `--tail <N>`: Number of lines to show from the end.
  - `--grep <KEYWORD>`: Filter lines by a keyword.

### `axiom gain`
Displays analytics on your token and cost savings.
- **Usage**: `axiom gain`
- **Flags**:
  - `--history`: Shows a detailed list of recent command executions and the exact token savings for each.

### `axiom discovery`
Lists or manages currently learned structural templates.
- **Subcommands**:
  - `list`: List all learned templates (default).
  - `clear`: Clear all learned patterns.
  - `forget <pattern>`: Forget a specific template pattern.

### `axiom check-ai`
Checks if the current process was called by an AI agent.
- **Usage**: `axiom check-ai`

### `axiom intent`
Manages the intelligence and relevance filtering levels.
- **Subcommands**:
  - `enable <mode>`: Enables intent-based filtering. Modes: `fuzzy` (default), `neural`.
  - `disable`: Sets intelligence to Level 1 (OFF). Only structure and privacy are processed.
  - `status`: Shows current intelligence mode and discovered intent.

## Configuration Commands

### `axiom config`
Configuration management.
- **Usage**: `axiom config [COMMAND]`
- **Behavior**: Without subcommands, it opens an interactive menu.
- **Subcommands**:
  - `init`: Initialize a local `.axiom.yaml` with default values.
  - `show`: Show current configuration.
  - `set <key> <value>`: Set a configuration value (e.g., `config set intelligence neural`).
