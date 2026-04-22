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
## Bolt Memory Optimization Log

*   **Gateway Stream Buffering (src/gateway/filters.rs):** Noticed that using `std::mem::take()` on `String` buffers replaces them with a zero-capacity default `String`, destroying pre-allocated capacities (e.g., `String::with_capacity(1024)`). Using `.clone()` followed by `.clear()` preserves the allocation and prevents reallocation spikes on high-frequency chunk streams.
*   **Insight Buffer Traversal (src/engine/commands/ps.rs):** In high-frequency iteration loops (e.g., mapping `top_proc`), allocating `String::new()` and cloning strings inside the loop creates unnecessary heap allocations. Using `Option<&str>` directly references the parent buffer item, keeping traversal operations zero-copy.
*   **Pipeline Deduplication Loop (src/engine/mod.rs):** Rather than unconditionally instantiating a new `String` for the `last_line` variable during every line processing event, extracting the buffer with `.take()`, resetting it with `.clear()`, and rewriting it with `.push_str(line)` eliminates dynamic allocations from the hottest loop inside Axiom's pipeline.
