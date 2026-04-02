# Axiom Memory Profile

## Memory Efficiency Audit
- **Data Structures (`DiscoveryEngine`)**: Replaced `HashMap` with `BTreeMap` for string-keyed buffers (`templates`, `synthesis_buffer`, `variable_buffer`). This mitigates the hashing overhead of small string keys and leverages the native sorting functionality, saving memory block sizes over frequent updates.
- **Buffer Flushing (`flush_variable_summary`)**: Swapped inefficient collection mappings with a combination of `std::mem::take` and direct iteration over `BTreeMap`'s inherent order. This completely eliminates large slice cloning and temporary heap vector allocations during output synchronization.
- **Regex Re-Compilation**: Pre-compiled all `Regex` instances inside `extract_parts` using `std::sync::OnceLock`. The lazy initialization eliminates continuous heap allocations and complex instruction branches on every processed line during active token compression streams.
- **Line Transforms (`apply_structural_transform`)**: Removed default `String` allocations by introducing `std::borrow::Cow<'a, str>`. Since most raw lines do not trigger formatting features (like markdown table parsing), utilizing borrowed references across the pipeline skips needless dynamic memory cloning on each input string.

All changes optimized strictly within Safe Rust.
