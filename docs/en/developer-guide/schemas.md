# Creating Schemas

Axiom relies on YAML schemas to understand the structural output of various CLI tools. These schemas define how to filter, collapse, and summarize repetitive or noisy logs.

Schemas are located in the `config/schemas/` directory.

## Anatomy of a Schema

A typical schema file (e.g., `npm.yaml`) looks like this:

```yaml
name: npm
description: "Node Package Manager"
command_pattern: "^(npm|npx)$"

rules:
  - name: npm_download_progress
    pattern: "^(?:npm WARN deprecated|npm notice|fetch|downloading)"
    action: collapse
    priority: 10
    summary: "[AXIOM] Collapsed {count} package fetching/warning logs."
    
  - name: npm_success_install
    pattern: "^added \\d+ packages"
    action: keep # Keep this through but maybe format it
    priority: 5
```

### Fields

- `name`: A human-readable name for the tool.
- `command_pattern`: A regular expression that matches the CLI executable that triggers this schema. If you type `axiom npm install`, Axiom matches `npm` against this pattern.
- `rules`: A list of matching rules applied to each line of output, ordered by `priority`.

### Rule Fields
- `name`: The identifier for the rule.
- `pattern`: The regex pattern to match against the output line.
- `action`: The transformation action to apply (see below).
- `priority`: An integer determining the execution order of the rules. Higher values are evaluated first.

### Rule Actions

- `keep`: Allows the line to print normally. Used to explicitly whitelist important lines.
- `collapse`: Hides the matching line. If multiple consecutive lines match, they are replaced by a single `summary` line. The `{count}` variable can be used in the summary.
- `redact`: Replaces sensitive information in the line with a redacted marker.
- `hidden`: Completely removes the line from the stream without any summary.
- `synthesize`: Groups multiple matching lines for intelligent summarization.

## How to Contribute a Schema

1. Create a new `.yaml` file in `config/schemas/`.
2. Identify the common "noise" patterns for the tool using regex.
3. Define `collapse` or `hidden` rules for the noise.
4. Test locally using `cargo run -- <your-command>`.
5. Submit a Pull Request!
