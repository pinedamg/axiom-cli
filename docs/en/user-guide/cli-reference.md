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
  - `--history`: Shows a detailed list of recent command executions and the exact token savings for each.

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

### `axiom config telemetry <level>`
Sets your preferred telemetry level.
- **Levels**: `full`, `discovery`, `anonymous`, `off` (Pro only).
- **Example**: `axiom config telemetry discovery`

### `axiom config license <key>`
Applies a Pro license key to unlock premium features like Offline telemetry mode.
- **Example**: `axiom config license abc-123-xyz`
