## Memory Profiling & Optimization - [Date]

### 🧩 Data Structure Audit & Optimization
*   **DiscoveryEngine Collections**: Analyzed `src/engine/discovery.rs` and replaced `HashMap` with `BTreeMap` for string-keyed collections (`templates`, `synthesis_buffer`, `variable_buffer`). This mitigates the heavy hashing overhead for small, localized string keys and natively leverages sorting which we require downstream.
*   **Buffer Flushing**: During summary generation `flush_variable_summary`, implemented `std::mem::take` to extract keys out of `BTreeMap` buffers. This prevents `O(N log N)` allocation overhead from cloning and manually sorting keys that were necessary with HashMaps. Additionally, pre-allocated the resulting `summaries` vector based on known lengths using `Vec::with_capacity` to prevent multi-allocation scaling inside the hot-path loop.
*   **Regex Statically Compiled**: Optimized Regex initializations in `extract_parts` by statically caching compiled Regex instances using `std::sync::OnceLock`. This heavily alleviates cyclic heap allocations and CPU cycles executing `Regex::new()` on almost every line inside the parsing loop.
*   **Cow Allocations in Transform**: Addressed `apply_structural_transform` returning dynamically allocated heap `String` strings on every processed line by restructuring to return `std::borrow::Cow<str>`. This safely handles string references and guarantees actual heap allocations only occur precisely when Markdown transformation is requested.

**Impact**: Significant prevention of unnecessary heap allocation and cyclic overhead inside the per-line 'hot path'. Lowered hashing memory costs for system tracking.

## Memory Profiling & Optimization - Pipeline Flow
1. **Pipeline Memory Overhead Reduction (`src/engine/mod.rs`)**
   - **Problem:** Every line processed through the pipeline (`stage_deduplicate`, `stage_guard`, `stage_analyze`) allocated a new `String` object via `line.to_string()`, generating significant heap allocation overhead in the hot path.
   - **Solution:** Replaced `PipelineAction` to use `std::borrow::Cow<'a, str>` instead of `String`.
   - **Impact:** When a line doesn't need redaction or transformation, it maintains its borrowed form all the way through the process, drastically reducing unnecessary allocations.

2. **Session Stats Consolidation (`src/engine/mod.rs`, `src/gateway/mod.rs`)**
   - **Problem:** Stats processing inside `ui.rs` relied on `get_session_stats`, but it was missing from `AxiomEngine`. `gateway/mod.rs` was maintaining these variables locally (`total_original`, `total_compressed`) instead of inside the core state.
   - **Solution:** Introduced a `SessionStats` struct tracking `raw_bytes` and `saved_bytes`. `AxiomEngine` now tracks these across its lifecycle natively, saving bytes during stream processing directly.

3. **WASM Plugin Manager Memory Footprint (`src/engine/plugins.rs`)**
   - **Problem:** Iterating over plugins unnecessarily cloned strings inside the loop.
   - **Solution:** Refactored `transform` to `process_line` and optimized string passing logic, reducing overhead when passing objects out of Rust back to the caller.
