# AXIOM: Development Log & Context Registry

This document records architectural decisions, current progress, and validation strategies to ensure project continuity.

## 1. Current Context (March 2026)
- **Phase**: Alpha/Beta (Phases 1, 2, and 3 completed as prototypes).
- **Status**: Interception system, privacy shield, intent logic, and auto-discovery engine functional.
- **Recent Milestone**: Empirical validation of "Auto-Discovery" with visual collapse feedback.

## 2. Validation Strategy (Testing)
To ensure Axiom is effective and does not "break" critical information for the LLM:

### Layer 1: Unit Tests (Atomic Logic)
- Validation of the entropy engine (secret detection).
- YAML schema matcher.

### Layer 2: Intent Simulation (Context Mocking)
- Use of `IntentContext` to simulate user prompts (e.g., "Find the error in X").
- Verification that the compressed output contains the necessary keywords to resolve the prompt.

### Layer 3: Token Benchmarking
- Each integration test must report token savings (Characters saved).
- Goal: >60% savings in noisy logs (npm, docker, maven).

## 3. AI Integration Strategy
Axiom must be transparent to the AI agent (Gemini, Claude, Cursor, etc.):

1.  **Proxy Interception**:
    - The agent executes `git status`.
    - The shell (via alias or hook) executes `axiom git status`.
    - Axiom cleans the output and returns it to the agent.
2.  **Intent Detection**:
    - Axiom will look for temporary files or environment variables where the agent stores the "Chat Context" to adjust its filter dynamically.

## 4. Key Design Decisions
- **Lib-First Architecture**: Axiom's core lives in `lib.rs` to facilitate testing.
- **Privacy-First**: Entropy scanning occurs before any semantic logic.
- **Visual Feedback (Phase 3+)**: A collapse counter was implemented to provide the user with visibility on token savings without losing the process status.

## 5. Visionary Tech Debt: Smart Aggregator (Phase 3/4 Discussion)
During Phase 3 validation, a critical improvement opportunity was identified ("Meat vs. Skeleton"):

- **Concept**: Not only hide noisy lines but synthesize them while maintaining their unique variables (IDs, Hashes, Names).
- **Mental Model**: If the engine detects a pattern like `Task #<NUM>: Processing <HEX>...`, instead of collapsing it to nothing, Axiom should capture the values of `<HEX>` in a buffer and generate a dense summary: `[AXIOM] Tasks 1-10 processed. IDs: [0x1, 0x2... 0x10]`.
- **Difficulty**: Documented as a **pluggable or future capability**, given that it requires a careful balance of latency and the risk of "token explosion" when grouping very long data.

---
*Last update: Completion of Phase 3 prototype and recording of Smart Aggregator.*
