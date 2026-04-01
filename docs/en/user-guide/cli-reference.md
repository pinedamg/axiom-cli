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
Install Axiom shell integration and AI context.
- **Usage**: `axiom install`
- **Flags**:
  - `--path <PATH>`: Project path to sync AI context (default: current dir).
  - `--context-only`: Only install AI context, skip shell aliases.

### `axiom uninstall`
Remove all Axiom traces from the system.
- **Usage**: `axiom uninstall`
- **Flags**:
  - `--path <PATH>`: Project path to remove AI context (default: current dir).

### `axiom doctor`
Run system health check and diagnostics.
- **Usage**: `axiom doctor`
- **Flags**:
  - `--path <PATH>`: Project path to check AI context (default: current dir).
  - `--fix`: Attempt to automatically fix detected issues.

### `axiom self-update`
Update Axiom to the latest version from GitHub.
- **Usage**: `axiom self-update`

### `axiom last`
Show the raw output of the last executed command.
- **Usage**: `axiom last`
- **Flags**:
  - `--tail <LINES>`: Number of lines to show from the end.
  - `--grep <KEYWORD>`: Filter lines by a keyword.

### `axiom check-ai`
Check if current process was called by an AI agent.
- **Usage**: `axiom check-ai`

### `axiom discovery`
List or manage currently learned structural templates.
- **Subcommands**:
  - `list`: List all learned templates (default).
  - `clear`: Clear all learned patterns.
  - `forget <PATTERN>`: Forget a specific template pattern.

### `axiom intent`
Manages the intelligence and relevance filtering levels.
- **Subcommands**:
  - `enable <mode>`: Enables intent-based filtering. Modes: `fuzzy` (default), `neural`.
  - `disable`: Sets intelligence to Level 1 (OFF). Only structure and privacy are processed.
  - `status`: Shows current intelligence mode and discovered intent.

### `axiom gain`
Displays analytics on your token and cost savings.
- **Usage**: `axiom gain`
- **Flags**:
  - `--history`, `-s`: Shows a detailed list of recent command executions and the exact token savings for each.

### `axiom status`
Shows the current health, configuration, and telemetry status of your Axiom installation.
- **Usage**: `axiom status`
- **Output**: Edition (Community/Pro), Telemetry Level, Installation ID, and active schemas.

### `axiom proxy <cmd>`
Executes the raw command without filtering. Useful for debugging or bypassing Axiom entirely for a specific execution.
- **Usage**: `axiom proxy npm install`

### `axiom discover`
*(Beta)* Analyzes local AI agent history (like Claude Code) to find missed opportunities where Axiom could have saved tokens.
- **Usage**: `axiom discover`

## Configuration Commands

### `axiom config`
Configuration management.
- **Subcommands**:
  - `init`: Initialize a local `.axiom.yaml` with default values.
  - `show`: Show current configuration.
  - `set <KEY> <VALUE>`: Set a configuration value (e.g., `config set intelligence neural`).
  - *(Interactive Mode)*: Running `axiom config` without subcommands opens an interactive menu to configure intelligence mode, markdown support, telemetry, privacy patterns, and intent sources.
