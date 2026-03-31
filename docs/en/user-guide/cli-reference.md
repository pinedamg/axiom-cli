# CLI Reference

The Axiom CLI provides several commands to manage your installation, view analytics, and debug configurations.

## Core Commands

### `axiom <command>`
The primary usage. Acts as a proxy for the provided command.
- **Usage**: `axiom npm install`, `axiom docker logs my-container`
- **Behavior**: Intercepts the command's output, applies privacy filters, semantic compression, and outputs the optimized stream.
- **Global Flags**:
  - `-m, --markdown`: Enables automatic transformation of terminal tables into Markdown format.
  - `-r, --raw`: Show raw output, bypassing all Axiom processing and synthesis. Outputs the exact stream from the child process.
  - `-y, --yes`: Automatically answer yes to all prompts.

### `axiom install`
Install Axiom shell integration and AI context.
- **Usage**: `axiom install`
- **Flags**:
  - `-p, --path <path>`: Project path to sync AI context (default: current dir).
  - `--context-only`: Only install AI context, skip shell aliases.

### `axiom uninstall`
Remove all Axiom traces from the system.
- **Usage**: `axiom uninstall`
- **Flags**:
  - `-p, --path <path>`: Project path to remove AI context (default: current dir).

### `axiom doctor`
Run system health check and diagnostics.
- **Usage**: `axiom doctor`
- **Flags**:
  - `-p, --path <path>`: Project path to check AI context (default: current dir).
  - `-f, --fix`: Attempt to automatically fix detected issues.

### `axiom self-update`
Update Axiom to the latest version from GitHub.
- **Usage**: `axiom self-update`

### `axiom last`
Show the raw output of the last executed command.
- **Usage**: `axiom last`
- **Flags**:
  - `-t, --tail <usize>`: Number of lines to show from the end.
  - `-g, --grep <string>`: Filter lines by a keyword.

### `axiom gain`
Show token savings analytics.
- **Usage**: `axiom gain`
- **Flags**:
  - `-s, --history`: Show detailed savings history.

### `axiom discovery`
List or manage currently learned structural templates.
- **Subcommands**:
  - `list` (default): List all learned templates.
  - `clear`: Clear all learned patterns.
  - `forget <pattern>`: Forget a specific template pattern.

### `axiom check-ai`
Check if current process was called by an AI agent.
- **Usage**: `axiom check-ai`

### `axiom intent`
Manage Intent Discovery and Intelligence Levels.
- **Subcommands**:
  - `enable <mode>`: Enable intent intelligence (`fuzzy` or `neural`, default: `fuzzy`).
  - `disable`: Disable intent intelligence (maintain formatting but show all files).
  - `status`: Show current intent discovery status and relevant files.

## Configuration Commands

### `axiom config`
Configuration management.
- **Usage**: `axiom config` (opens an interactive menu).
- **Subcommands**:
  - `init`: Initialize a local `.axiom.yaml` with default values.
  - `show`: Show current configuration.
  - `set <key> <value>`: Set a configuration value (e.g. `axiom config set intelligence neural`). Supported keys: `intelligence`, `markdown`.
