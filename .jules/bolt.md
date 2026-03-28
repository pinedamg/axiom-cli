# Bolt: Memory & Performance Optimization Log

## Issue: Overhead in Regex Compilation
- **Finding:** The `extract_parts` function in `src/engine/discovery.rs` was recompiling regular expressions for every processed line. Since this is in the "hot path," compiling Regexes dramatically increased line-processing latency and allocated unnecessary memory.
- **Mitigation:** We utilized `std::sync::OnceLock` to cache the `Regex` instances. This static caching avoids recompiling patterns for every line and decreases overhead.

## Issue: Suboptimal Hash Structures
- **Finding:** Axiom used `HashMap` for string-keyed collections like `templates`, `synthesis_buffer`, and `variable_buffer` within the `DiscoveryEngine`. The standard library `HashMap` comes with an expensive hashing overhead suitable for DOS resistance, but overkill for local CLI tools. Additionally, summarizing keys dynamically required vector allocations and an `O(n log n)` sort.
- **Mitigation:** Substituted `HashMap` with `BTreeMap`. `BTreeMap` guarantees ordered traversal with zero allocations or intermediate clones, solving both the expensive hashing and dynamic sorting overhead. We updated the iterators to use `std::mem::take` to extract data naturally sorted by keys.

## Issue: Frequent Heap Allocations
- **Finding:** Generating reports inside `flush_variable_summary` triggered iterative heap reallocations when growing dynamic arrays, wasting CPU and RAM.
- **Mitigation:** Added `Vec::with_capacity` pre-allocations utilizing the exact buffer sizing length from `synthesis_buffer` and `variable_buffer` to eliminate dynamic resizing.

## Issue: False Positives on Redaction
- **Finding:** `PrivacyRedactor` frequently redacted safe context like Git hashes (40 char HEX) and Docker IDs (64 char HEX), allocating memory to generate `<REDACTED_SECRET>` unnecessarily.
- **Mitigation:** We updated the logic to preemptively return strings containing precisely 40 or 64 valid ascii hex characters. Returning exactly the parsed hash strings natively ensures context preservation and speeds up analysis.
