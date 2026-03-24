# Bolt: Memory Profiling & Optimization Journal

## 🔍 Memory Profiling (Static Analysis)

**Finding:**
During the analysis of `src/engine/discovery.rs`, a massive source of redundant allocation and CPU cycles was identified in the `extract_parts` function. The function was parsing and compiling 5 distinct `Regex` patterns on *every single line* processed by Axiom.
Additionally, `variable_buffer` and `templates` within `DiscoveryEngine` utilized `HashMap`, which allocates larger amounts of memory even for small amounts of items due to hashing overhead compared to `BTreeMap`.
Finally, `src/engine/mod.rs` returned an owned `String` in `apply_structural_transform` even when the line wasn't modified, creating an allocation per line.

## 🛠️ Refactoring for Efficiency

**Changes Implemented:**

1.  **Regex Caching (`OnceLock`):**
    *   **What:** The 5 regular expressions in `extract_parts` (`RE_UUID`, `RE_PATH`, `RE_TIME`, `RE_HEX`, `RE_NUM`) were moved to statically initialized `std::sync::OnceLock`.
    *   **Why:** Compiling a `Regex` in Rust is an expensive operation in terms of both memory and CPU time. Doing this inside a loop or per-line significantly degrades performance and causes unnecessary short-lived heap allocations (RAM thrashing).
    *   **Impact:** Drastic reduction in CPU overhead and temporary memory allocations per processed line. Instead of millions of Regex compilations for large logs, there are exactly 5.

2.  **`Vec` Capacity Pre-allocation:**
    *   **What:** Initialized the `variables` vector in `extract_parts` with `Vec::with_capacity(4)`.
    *   **Why:** A line usually contains only a handful of extractable variables. Pre-allocating prevents the `Vec` from resizing (and re-allocating heap memory) multiple times as items are pushed.
    *   **Impact:** Avoids small heap re-allocations in the hot path.

3.  **`BTreeMap` over `HashMap`:**
    *   **What:** Replaced `HashMap` with `BTreeMap` for the `templates` and `variable_buffer` in `DiscoveryEngine`.
    *   **Why:** `HashMap` has a larger memory footprint for a small number of entries due to its internal layout and hashing overhead. `BTreeMap` is more compact and efficient for string keys in scenarios where map sizes are relatively small or predictability in layout is preferred.
    *   **Impact:** Reduced memory footprint per `DiscoveryEngine` instance.

4.  **`Cow<'_, str>` Usage:**
    *   **What:** `apply_structural_transform` in `src/engine/mod.rs` was refactored to return `std::borrow::Cow<'_, str>` instead of `String`.
    *   **Why:** The transformation only actually allocates and generates a new string if markdown mode is enabled AND the line looks like a table. In 99% of other cases, it was unnecessarily allocating an exact clone of the input line string.
    *   **Impact:** A guaranteed heap allocation per line processed was completely eliminated in the common case.

## 📉 Benchmarking of Resources (Estimated Impact)

*   **Before:** Processing a 1,000,000 line log file would result in:
    *   5,000,000 `Regex` compilations (massive CPU/RAM spike).
    *   1,000,000 redundant `String` allocations for `apply_structural_transform`.
    *   Countless `Vec` resizing re-allocations.
*   **After:**
    *   Exactly 5 `Regex` compilations.
    *   0 redundant `String` allocations for normal lines.
    *   0 `Vec` re-allocations (for lines with <= 4 variables).
    *   More tightly packed memory via `BTreeMap`.

Overall, this provides a significantly flatter RAM usage profile and avoids sudden GC/allocator pauses during heavy streaming.
