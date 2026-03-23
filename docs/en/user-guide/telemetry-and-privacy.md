# Telemetry & Privacy

Axiom is built for privacy. We collect anonymous metadata to improve compression algorithms and discover missing command schemas. **No command arguments, secrets, or PII ever leave your machine.**

## Telemetry Levels

Axiom offers four transparency levels, configurable via:
```bash
axiom config telemetry <level>
```

1. **`Full` (Default)**: Shares anonymous savings metrics + Binary names used + Internal metrics (rule match IDs).
2. **`Discovery`**: Shares anonymous savings metrics + Binary names (e.g., `git`, `npm`). This helps us prioritize which new tool schemas to build next.
3. **`Anonymous`**: Only aggregates token savings, your OS, and Axiom version. No command names are sent.
4. **`Off`**: **PRO FEATURE**. Total blackout. No data is ever sent from your machine.

## Transparency First

Run `axiom status` at any time to see exactly what data is being shared and your current edition (Community vs Pro).

## How We Protect You

- **Command Sanitization**: Even in `Full` mode, we **ONLY** capture the binary name (e.g., `npm`), never the arguments (e.g., `install secret-package`).
- **Anonymous ID**: We use a random `installation_id` to count active instances without knowing who you are.
- **Pro Privacy**: Support Axiom by upgrading to Pro to unlock the `Off` telemetry mode.
