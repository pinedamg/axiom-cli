# Creating Schemas

Axiom relies on YAML schemas to understand the structural output of various CLI tools. These schemas define how to filter, collapse, and summarize repetitive or noisy logs.

Schemas are located in the `config/schemas/` directory.

## Anatomy of a Schema

A typical schema file (e.g., `npm.yaml`) looks like this:

```yaml
name: npm
command_pattern: "^(npm|npx)"
rules:
  - name: DownloadProgress
    pattern: "^(?:npm WARN deprecated|npm notice|fetch|downloading)"
    action: collapse
    priority: 5
    
  - name: SuccessInstall
    pattern: "^added \\d+ packages"
    action: keep
    priority: 1

  - name: SensitiveToken
    pattern: "^.*token=.*"
    action: redact
    priority: 10
```

### Fields

- `name`: A human-readable name for the tool.
- `command_pattern`: A regular expression used to match the executed command.
- `rules`: A list of matching rules applied sequentially to each line of output.

### Rule Actions

- `keep`: Allows the line to print normally. Used to explicitly whitelist important lines.
- `collapse`: Hides the matching line. If multiple consecutive lines match, they are replaced by a single summary line.
- `redact`: Masks sensitive information in the line before further processing.
- `hidden`: Completely removes the line from the stream without any summary.
- `synthesize`: Groups multiple matching lines into a condensed, intelligent summary.

## How to Contribute a Schema

1. Create a new `.yaml` file in `config/schemas/`.
2. Identify the common "noise" patterns for the tool using regex.
3. Define `collapse` or `hidden` rules for the noise.
4. Test locally using `cargo run -- <your-command>`.
5. Submit a Pull Request!
