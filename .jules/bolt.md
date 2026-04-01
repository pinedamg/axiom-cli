# Axiom Memory Footprint & Efficiency Journal

## ⚡ Bolt Optimizations

### 1. BTreeMap vs HashMap for String Keys
**Context**: `DiscoveryEngine` relies heavily on storing parsed template lines, variables and synthesis objects categorized by String keys (e.g., `GIT:LOG_COMMIT:ALL`).
**Problem**: Using `HashMap` incurs hashing overhead for string comparisons on every log line processing (hot path). Furthermore, `flush_variable_summary` requires the output to be lexicographically sorted. To achieve this, the entire map's keys were cloned and a `Vec` was allocated just to sort the keys, creating excessive intermediate allocations on the heap.
**Solution**: Transitioned `templates`, `synthesis_buffer`, and `variable_buffer` to `BTreeMap`. `BTreeMap` is natively sorted. This allows using `std::mem::take` during flushing, transferring memory ownership out of the buffer in `O(1)` allocations and completely bypassing the need to allocate key vectors and copy strings to format the terminal stream output.

### 2. Regex Engine Static Caching in the Pipeline
**Context**: The function `extract_parts` uses `Regex` patterns to find and tokenize high-entropy strings like UUIDs, HEX codes, and paths to create structural templates.
**Problem**: The `Regex::new(r"...")` invocations were repeatedly firing on every individual line read from a child process stdout. The Rust Regex engine performs expensive DFA compilation at runtime, severely limiting Axiom's sub-10ms CLI latency objective and bloating CPU caches and memory.
**Solution**: Refactored `extract_parts` to implement `std::sync::OnceLock`. The 6 regex patterns are now statically compiled once globally on the first parsed line and re-used for all subsequent logs.

### 3. Pre-allocation of Vectors
**Context**: `extract_parts` was instantiating dynamic vectors for matched template variables without defined capacities.
**Problem**: If multiple variables were identified on a line, the heap was forced to re-allocate vector bounds dynamically.
**Solution**: Pre-allocated variable vectors with `Vec::with_capacity(4)` reducing dynamic resizing frequency for the most common use-cases.
