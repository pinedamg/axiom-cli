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

## Memory Consumption Patterns (Discovery Engine & Pipelines)

* **Zero-capacity buffer replacement overhead:** In high-frequency stream filters (e.g., `StreamPipeline::process` in `src/gateway/filters.rs`), using `std::mem::take` on pre-allocated strings replaces them with zero-capacity buffers. This causes massive reallocation overhead. We optimized this by using `std::mem::replace(&mut buffer, String::with_capacity(CAPACITY))` to explicitly maintain memory capacity during string extraction.
* **String buffer reuse:** In high-frequency line processing (e.g., `stage_deduplicate`), allocating new strings constantly (like `Some(line.to_string())`) generates heavy heap traffic. We optimized this by reusing existing `Option<String>` buffers, extracting them with `.take()`, clearing them with `.clear()`, and appending new data with `.push_str()`.
* **Regex Variable Collection Overhead:** Axiom's `DiscoveryEngine` was extracting and storing exact variable content from Regex matches (e.g., UUIDs, Paths, Hex) inside a `BTreeMap<String, Vec<Vec<String>>>` to track matches. During high-frequency streams (like large logs or heavily templated loops), this caused severe heap allocations and memory bloat. We optimized this by completely dropping variable collection and just incrementing a `usize` counter (`BTreeMap<String, usize>`) since the exact variable values were not actively utilized downstream, saving substantial memory per matched line.
* **Accurate BTreeMap memory footprints:** When estimating the memory footprint of `BTreeMap<String, usize>` structures in Axiom (e.g., `DiscoveryEngine::get_saved_bytes`), it is more accurate to use `template.capacity() + std::mem::size_of::<usize>()` rather than multiplying string lengths by match counts, avoiding artificially inflated memory saving metrics.
