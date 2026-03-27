# CLI Reference

The Axiom CLI provides several commands to manage your installation, view analytics, and debug configurations.

## Core Commands

### `axiom <command>`
The primary usage. Acts as a proxy for the provided command.
- **Usage**: `axiom npm install`, `axiom docker logs my-container`
- **Behavior**: Intercepts the command's output, applies privacy filters, semantic compression, and outputs the optimized stream.
- **Flags**:
  - `--markdown` / `-m`: Enable Markdown table transformation for structured outputs.

### `axiom gain`
Displays analytics on your token and cost savings.
- **Usage**: `axiom gain`
- **Flags**:
  - `--history`: Shows a detailed list of recent command executions and the exact token savings for each.

### `axiom status`
Shows the current health, configuration, and telemetry status of your Axiom installation.
- **Usage**: `axiom status`
- **Output**: Edition (Community/Pro), Telemetry Level, Installation ID, and active schemas.

### `axiom proxy <cmd>`
Executes the raw command without filtering. Useful for debugging or bypassing Axiom entirely for a specific execution.
- **Usage**: `axiom proxy npm install`

### `axiom discovery`
List currently learned structural templates.
- **Usage**: `axiom discovery`

### `axiom check-ai`
Check if the current process was called by an AI agent or a human shell.
- **Usage**: `axiom check-ai`

### `axiom intent <action>`
Manage Intent Discovery and Intelligence Levels.
- **Usage**: `axiom intent enable [mode]`, `axiom intent disable`, `axiom intent status`
- **Actions**:
  - `enable [mode]`: Enable intent intelligence (fuzzy or neural). Default is fuzzy.
  - `disable`: Disable intent intelligence (maintain formatting but show all files).
  - `status`: Show current intent discovery status and relevant files.

## Configuration Commands

### `axiom config init`
Initialize a local `.axiom.yaml` configuration file with default values.
- **Usage**: `axiom config init`

### `axiom config telemetry <level>`
Sets your preferred telemetry level.
- **Levels**: `full`, `discovery`, `anonymous`, `off` (Pro only).
- **Example**: `axiom config telemetry discovery`

### `axiom config license <key>`
Applies a Pro license key to unlock premium features like Offline telemetry mode.
- **Example**: `axiom config license abc-123-xyz`
