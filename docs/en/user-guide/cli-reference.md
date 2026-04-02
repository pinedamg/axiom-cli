# CLI Reference

The Axiom CLI provides several commands to manage your installation, view analytics, and debug configurations.

## Global Flags

- `--raw`: Bypasses all Axiom processing and synthesis. Outputs the exact stream from the child process.
- `--markdown`: Enables automatic transformation of terminal tables into Markdown format.
- `--yes`: Automatically answer yes to all prompts.

## Core Commands

### `axiom <command>`
The primary usage. Acts as a proxy for the provided command.
- **Usage**: `axiom npm install`, `axiom docker logs my-container`
- **Behavior**: Intercepts the command's output, applies privacy filters, semantic compression, and outputs the optimized stream.

### `axiom install`
Install Axiom shell integration and AI context.
- **Flags**:
  - `--path <PATH>`: Project path to sync AI context (default: current dir)
  - `--context-only`: Only install AI context, skip shell aliases

### `axiom uninstall`
Remove all Axiom traces from the system.
- **Flags**:
  - `--path <PATH>`: Project path to remove AI context (default: current dir)

### `axiom doctor`
Run system health check and diagnostics.
- **Flags**:
  - `--path <PATH>`: Project path to check AI context (default: current dir)
  - `--fix`: Attempt to automatically fix detected issues

### `axiom selfupdate`
Update Axiom to the latest version from GitHub.

### `axiom last`
Show the raw output of the last executed command.
- **Flags**:
  - `--tail <NUMBER>`: Number of lines to show from the end
  - `--grep <KEYWORD>`: Filter lines by a keyword

### `axiom gain`
Displays analytics on your token and cost savings.
- **Flags**:
  - `-s, --history`: Shows a detailed list of recent command executions and the exact token savings for each.

### `axiom discovery`
List or manage currently learned structural templates.
- **Subcommands**:
  - `list` (default): List all learned templates.
  - `clear`: Clear all learned patterns.
  - `forget <PATTERN>`: Forget a specific template pattern.

### `axiom checkai`
Check if the current process was called by an AI agent.

### `axiom config`
Configuration management.
- **Subcommands**:
  - `init`: Initialize a local `.axiom.yaml` with default values.
  - `show`: Show current configuration.
  - `set <KEY> <VALUE>`: Set a configuration value (e.g., `config set intelligence neural`).

### `axiom intent`
Manage Intent Discovery and Intelligence Levels.
- **Subcommands**:
  - `enable <MODE>`: Enables intent-based filtering. Modes: `fuzzy` (default), `neural`.
  - `disable`: Sets intelligence to Level 1 (OFF). Only structure and privacy are processed.
  - `status`: Shows current intent discovery status and relevant files.
