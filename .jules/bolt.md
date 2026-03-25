# Axiom Memory Optimization Journal (Bolt)

## Memory Footprint Profiling & Findings

- **Data Structure Optimization**: Analysed `src/engine/discovery.rs` and identified memory accumulation via `HashMap` utilization in `templates`, `synthesis_buffer`, and `variable_buffer`. Replaced them with `BTreeMap`. This avoids hashing overhead for small, localized collections inside `DiscoveryEngine` and inherently keeps data sorted, avoiding the need for an extra sort operation and vector allocation in `flush_variable_summary`.
- **Regex Compilation Overhead**: Found continuous Regex compilation in the hot path of `extract_parts()`. Adopted `std::sync::OnceLock` to statically cache these regexes, cutting down the continuous memory/CPU cycle toll per line processed.
- **Allocation Reductions**: `extract_parts` uses a vector with an initially undefined capacity, requiring continuous reallocation. Introduced `Vec::with_capacity(4)` pre-allocation since most lines don't possess many matches.
- **Buffer Cleanup Strategy**: Refactored `flush_variable_summary` buffer read to implement `std::mem::take`, effectively emptying buffers and consuming them without cloning large nested structures.
- **String Cloning in Hot Path**: Found repetitive string allocations in `src/engine/mod.rs` where `self.last_command` was cloned per line without checking if it actually mutated. Refactored string cloning behind a change validator (`if self.last_command != command`).

These updates collectively reinforce memory safety while yielding extreme efficiency inside Axiom's real-time streaming constraints.