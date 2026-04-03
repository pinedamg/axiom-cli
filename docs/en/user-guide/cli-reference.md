# CLI Reference

The Axiom CLI provides several commands to manage your installation, view analytics, and debug configurations.

When you run `axiom` followed by any command not listed below, it acts as a **Semantic Firewall**, intercepting the command's output, applying privacy filters and semantic compression, and outputting an optimized, high-signal stream.

## Proxy Flags

When running in proxy mode (e.g., `axiom npm install`), you can use the following global flags:

- `--raw`: Bypasses all Axiom processing and synthesis. Outputs the exact stream from the child process.
- `--markdown`: Enables automatic transformation of terminal tables into Markdown format.
- `--yes`: Automatically answer "yes" to all prompts.

## Core Commands

### `axiom install`
Install Axiom shell integration and AI context.
- **Flags**:
  - `-p, --path <PATH>`: Project path to sync AI context (default: current dir).
  - `--context-only`: Only install AI context files (e.g., `AGENTS.md`, `.cursorrules`), skip shell aliases.

### `axiom uninstall`
Remove all Axiom traces from the system.
- **Flags**:
  - `-p, --path <PATH>`: Project path to remove AI context (default: current dir).

### `axiom doctor`
Run system health check and diagnostics.
- **Flags**:
  - `-p, --path <PATH>`: Project path to check AI context (default: current dir).
  - `-f, --fix`: Attempt to automatically fix detected issues.

### `axiom self-update`
Update Axiom to the latest version from GitHub.

### `axiom last`
Show the raw output of the last executed command.
- **Flags**:
  - `-t, --tail <LINES>`: Number of lines to show from the end.
  - `-g, --grep <KEYWORD>`: Filter lines by a keyword.

### `axiom gain`
Show token savings analytics.
- **Flags**:
  - `-s, --history`: Show detailed savings history.

### `axiom check-ai`
Check if the current process was called by an AI agent. Exits with 0 if detected, 1 otherwise.

## Configuration & Discovery Commands

### `axiom intent <action>`
Manage Intent Discovery and Intelligence Levels.
- **Actions**:
  - `enable <mode>`: Enable intent intelligence. Modes: `fuzzy` (keywords) or `neural` (AI embeddings). Default is `fuzzy`.
  - `disable`: Disable intent intelligence (maintain formatting but show all files).
  - `status`: Show current intent discovery status and relevant files.

### `axiom discovery <action>`
List or manage currently learned structural templates.
- **Actions**:
  - `list` *(default)*: List all learned templates.
  - `clear`: Clear all learned patterns.
  - `forget <pattern>`: Forget a specific template pattern.

### `axiom config <action>`
Configuration management. If no action is provided, an interactive configuration menu is launched.
- **Actions**:
  - `init`: Initialize a local `.axiom.yaml` file with default values.
  - `show`: Show current configuration.
  - `set <key> <value>`: Set a configuration value (e.g., `axiom config set intelligence neural`).
