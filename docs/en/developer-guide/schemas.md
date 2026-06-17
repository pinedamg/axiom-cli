# Creating Schemas

Axiom relies on YAML schemas to understand the structural output of various CLI tools. These schemas define how to filter, collapse, and summarize repetitive or noisy logs.

Schemas are located in the `config/schemas/` directory.

## Anatomy of a Schema

A typical schema file (e.g., `npm.yaml`) looks like this:

```yaml
name: npm
description: "Node Package Manager"
binary_names:
  - npm
  - npx

rules:
  - id: npm_download_progress
    pattern: "^(?:npm WARN deprecated|npm notice|fetch|downloading)"
    action: collapse
    summary: "[AXIOM] Collapsed {count} package fetching/warning logs."
    
  - id: npm_success_install
    pattern: "^added \\d+ packages"
    action: keep # Pass this through but maybe format it
```

### Fields

- `name`: A human-readable name for the tool.
- `binary_names`: A list of CLI executables that trigger this schema. If you type `axiom npm install`, Axiom matches `npm` against this list.
- `rules`: A list of matching rules applied sequentially to each line of output.

### Rule Actions

- `keep`: Allows the line to print normally. Used to explicitly whitelist important lines.
- `collapse`: Hides the matching line. If multiple consecutive lines match, they are replaced by a single `summary` line. The `{count}` variable can be used in the summary.
- `redact`: Completely removes the line and replaces it with `[REDACTED_BY_SCHEMA]` for privacy shielding.
- `hidden`: Completely removes the line from the stream without any summary.
- `synthesize`: The new action for intelligent grouping, replacing multiple matched lines with a smart summary.

## How to Contribute a Schema

1. Create a new `.yaml` file in `config/schemas/`.
2. Identify the common "noise" patterns for the tool using regex.
3. Define `collapse` or `drop` rules for the noise.
4. Test locally using `cargo run -- <your-command>`.
5. Submit a Pull Request!
