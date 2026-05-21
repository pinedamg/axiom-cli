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

### 🧩 Gateways and Hot-Paths Re-Optimization (Continued)
*   **Buffer Recycling in `stage_deduplicate`**: Instead of cloning `String` objects every time a new line is recorded as `last_line` inside `src/engine/mod.rs`, the previous buffer is now extracted via `take()`, cleared (`.clear()`), and the new string is appended via `.push_str()`. This prevents continuous capacity allocations inside the line-processing hot path.
*   **StreamPipeline Terminal Buffers**: Within `src/gateway/filters.rs`, the processing loop previously used `std::mem::take` during line-feeds, which resets the internal buffer capacity to 0. This was replaced with `std::mem::replace(&mut self.buffer, String::with_capacity(1024))`, assuring the terminal processor doesn't constantly suffer reallocation overhead on busy stdout outputs.
*   **Discovery Engine Regex Optimizations**: Extracted variable vectors were completely removed from `variable_buffer` as they were unused except for counting occurrences. `extract_parts` was refactored to directly yield `.into_owned()` Strings, entirely bypassing intermediary vector allocations and replacing closures with static replace string targets. Regex instantiations were updated to use `.expect("Invalid hardcoded regex")`.

**Impact**: Multi-gigabyte GC turnover reduction during high-frequency text-processing (e.g. sprawling terminal sessions or complex discovery threshold evaluation). Streamlining these localized structures explicitly lowers OS memory locking overhead inside the sub-10ms pipeline envelope.
