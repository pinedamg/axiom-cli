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

### ⚡ Hot-Path Optimizations
*   **Buffer Re-use (`src/engine/mod.rs`)**: Replaced `self.discovery.last_line = Some(line.to_string())` with a strategy that extracts the buffer using `.take()`, clears it, and appends via `.push_str()` to reuse allocations.
*   **Capacity Retention (`src/gateway/filters.rs`)**: Replaced `std::mem::take()` with `.clone()` followed by `.clear()` in the `StreamPipeline::process` loop to preserve pre-allocated buffer capacities.
*   **Pointer Tracking (`src/engine/commands/ps.rs`)**: Switched from `String` cloning to tracking an `Option<&str>` referencing discovery buffers to avoid unnecessary string copies in insight generation.

**Impact**: Significant reductions in memory allocations and reallocations in the gateway event stream and the deduplication hot-path.
