## Memory Profiling & Optimization - [Date]

### 🧩 Data Structure Audit & Optimization
*   **DiscoveryEngine Collections**: Analyzed `src/engine/discovery.rs` and replaced `HashMap` with `BTreeMap` for string-keyed collections (`templates`, `synthesis_buffer`, `variable_buffer`). This mitigates the heavy hashing overhead for small, localized string keys and natively leverages sorting which we require downstream.
*   **Buffer Flushing**: During summary generation `flush_variable_summary`, implemented `std::mem::take` to extract keys out of `BTreeMap` buffers. This prevents `O(N log N)` allocation overhead from cloning and manually sorting keys that were necessary with HashMaps. Additionally, pre-allocated the resulting `summaries` vector based on known lengths using `Vec::with_capacity` to prevent multi-allocation scaling inside the hot-path loop.
*   **Regex Statically Compiled**: Optimized Regex initializations in `extract_parts` by statically caching compiled Regex instances using `std::sync::OnceLock`. This heavily alleviates cyclic heap allocations and CPU cycles executing `Regex::new()` on almost every line inside the parsing loop.
*   **Cow Allocations in Transform**: Addressed `apply_structural_transform` returning dynamically allocated heap `String` strings on every processed line by restructuring to return `std::borrow::Cow<str>`. This safely handles string references and guarantees actual heap allocations only occur precisely when Markdown transformation is requested.

**Impact**: Significant prevention of unnecessary heap allocation and cyclic overhead inside the per-line 'hot path'. Lowered hashing memory costs for system tracking.

---

## Memory Profiling & Optimization - Gateways and Hot-Paths

### 🧩 Data Structure Audit & Optimization
*   **Hot-Path Allocations (`src/engine/mod.rs`)**: Discovered that passing lines down the axiom stream pipeline caused multiple `String` heap allocations on every tick. Refactored `PipelineAction` to use `Cow<'a, str>` instead of strict `String`s. This allowed zero-allocation pass-throughs when stages do not fundamentally modify the text (e.g. `stage_deduplicate`, `stage_guard`, `stage_analyze`). Because lines generally outlive the match scope, strings are bound locally and mapped appropriately to ensure lifetimes are satisfied without sacrificing borrow benefits when feasible.
*   **Terminal Gateway Overhead (`src/gateway/filters.rs`)**: Initialized `StreamPipeline.buffer` with `String::with_capacity(1024)` based on an estimated typical dense line length. `events` vector capacity pre-allocated to 16 based on average chunk iterations.
*   **Pattern Matching RegEx (`src/engine/discovery.rs`)**: Extracted variables matched by privacy RegEx constructs iteratively appended to an unconstrained vector, which forced resizing on noisy unstructured strings. Refactored `extract_parts` to initialize the `variables` vector with `Vec::with_capacity(8)`.

**Impact**: Expected multi-megabyte GC/heap turnover reduction per minute during dense log streams (e.g., recursive `ls`, intensive `npm install`, sprawling `cargo build`). Pre-allocations should significantly decrease OS memory locking overhead inside the sub-10ms performance envelope.

### 🧩 Data Structure Audit & Optimization - Command Handlers
*   **Trait Signature Update (`src/engine/commands/mod.rs`)**: Changed `CommandHandler::get_category` to accept `&LineMetadata` instead of `&str`. This change ensures command handlers can utilize full context (such as the `is_dir` flag set during parsing) to accurately output the category grouping prefix.
*   **Hot-Path Avoidance & Panic Prevention (`src/engine/commands/ps.rs`)**: The tracking of kernel vs. process prefixes in `PsHandler` has been successfully implemented correctly inside `get_category` via `meta.is_dir`. Handled potential runtime crashes by swapping `str::split().unwrap()` with safer variants (`.unwrap_or(base)`), ensuring panics on out-of-bounds parsing errors do not halt telemetry.
*   **Zero-Copy Insight Tracking (`src/engine/commands/ps.rs`)**: Inside `PsHandler::generate_insight`, `top_proc` which represents the currently scanned heavily utilized process name is refactored from `String` to `Option<&str>`. This effectively drops recursive allocation inside the inner search loop iterating over discovery buffer references, fetching data straight from memory instead of blindly allocating `clone()` objects.

**Impact**: Safety, consistency, and allocation scaling on large ps trace lines. Handlers execute more predictably without edge panic crashes, saving time in continuous telemetry and lowering RAM overhead over dense streams of logs.
