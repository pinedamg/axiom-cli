# AXIOM: Developer Toolset Roadmap

This specialized roadmap defines the expansion of default schemas and intelligent modes for Linux developers.

## 🟢 Tier 1: The Core Fundamentals (High Frequency)
*Goal: Remove structural noise from everyday commands.*

- [x] **ls / tree**: Collapse hidden files, metadata (`total X`), and junk directories (`node_modules`, `.git`).
- [ ] **cat / tail / head**: Implement "Guardian Mode" for files > 50 lines (auto-summary).
- [ ] **grep / rg (ripgrep)**: Aggregate matches per file and provide density summaries.
- [ ] **curl / wget**: Hide progress bars and redundant HTTP headers.

## 🟡 Tier 2: Build & Dev Ecosytems (Context-Aware)
*Goal: Filter successful boilerplate and focus on user-authored code warnings/errors.*

- [x] **npm / pnpm / yarn**: Basic installer noise reduction (In Progress).
- [ ] **cargo (Rust)**: Collapse dependency downloading/compiling. Force-show local crate warnings.
- [ ] **go build / test**: Summarize test results (e.g., "400 passed, 2 failed").
- [ ] **pip / poetry / conda**: Clean virtualenv setup and package installation logs.

## 🟠 Tier 3: Infrastructure & Cloud (Volume Control)
*Goal: Prevent context window saturation from massive infrastructure outputs.*

- [x] **docker / docker-compose**: Collapse layer pull progress and health-check loops.
- [ ] **kubectl**: Summarize pod states, clean resource descriptions, and remove irrelevant uptime columns.
- [ ] **terraform**: Synthesize `terraform plan` (show only resource deltas).
- [ ] **aws / gcloud / az**: Transform massive JSON/Table listings into dense summaries.

## 🔵 Tier 4: Data & System (Structural Synthesis)
*Goal: Maintain data shape while reducing token count.*

- [ ] **jq / yq**: Identify JSON structure and summarize large arrays.
- [x] **ps / journalctl**: Deep cleaning of system/kernel noise (In Progress).
- [ ] **netstat / lsof / ss**: Filter system-reserved ports and focus on user-app connections.

---

## 🚀 Advanced Intelligent Modes (Behavioral Flags)

- **`--markdown`**: Automatically transform table outputs (docker, ps, ls) into real Markdown tables for better LLM comprehension.
- **`--diff-only`**: Persistent state matching to show only what changed since the last execution of the same command.
- **`--explain`**: Prepend a natural language summary of what Axiom compressed.

---
*"A tool is only as good as the noise it ignores."*
