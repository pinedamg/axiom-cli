# Quick Start

Axiom acts as a proxy command. To use it, simply prefix your usual terminal commands with `axiom`.

## Basic Usage

Whenever you run a command that outputs a lot of noise, prefix it:

```bash
axiom npm install
```

```bash
axiom git diff
```

```bash
axiom docker-compose up
```

## How It Works

1. **Interception**: Axiom captures the `stdout` and `stderr` of the command.
2. **Analysis**: It identifies the tool being run (e.g., `npm`) and applies the corresponding parsing rules.
3. **Compression**: Repetitive noise is stripped, and structurally similar logs are summarized into a single dense line.
4. **Protection**: Before anything is printed to the screen, Axiom's Privacy Shield scans for high-entropy strings (like API keys) and redacts them.

## Checking Your Savings

Axiom keeps track of how many tokens it has saved you locally.

```bash
axiom gain
```

To see a detailed history of your token savings per command:

```bash
axiom gain --history
```

## Adjusting Intelligence

Depending on your task, you can change how aggressively Axiom filters output:

*   **Deep Debugging**: `axiom intent enable neural` (Uses local AI embeddings).
*   **Standard Mode**: `axiom intent enable fuzzy` (Default, keyword-based).
*   **Just Summaries**: `axiom intent disable` (No AI relevance filtering).
*   **Total Bypass**: `axiom --raw <command>` (Raw output).
