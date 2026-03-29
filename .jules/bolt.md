# Axiom Memory Optimizations - Bolt Log

## Optimizations applied:

1. **HashMap to BTreeMap in DiscoveryEngine**: Replaced `HashMap` with `BTreeMap` for `templates`, `synthesis_buffer`, and `variable_buffer`.
   - *Why*: Reduces hashing overhead since many keys in Axiom are small and well-structured strings.
   - *Impact*: Lowers intermediate memory usage and avoids an additional step allocating and sorting vector arrays.

2. **Zero-Copy Draining of BTreeMaps**: Refactored `flush_variable_summary` to use `std::mem::take` to iterate and extract data from buffers directly.
   - *Why*: Avoids `keys().cloned().collect()` into an intermediate `Vec` to perform an expensive sort operation, mitigating unnecessary heap allocations.

3. **Static Regex Pattern Caching**: Used `std::sync::OnceLock` for Regex patterns in `extract_parts`.
   - *Why*: Regex compilation during the line-processing hot path takes significant CPU time and memory reallocation for the state machine.
   - *Impact*: Achieves sub-10ms CLI latency by caching dynamically compiled patterns. Pre-allocated capacity further prevents reallocation overhead on the matches.

4. **Copy-on-Write for Structural Transformation**: Updated `apply_structural_transform` in `AxiomEngine` to return `std::borrow::Cow<'_, str>` instead of `String`.
   - *Why*: The vast majority of lines during command outputs do not undergo structural table transforms (markdown format). Generating an owned string every time caused severe heap churn.
   - *Impact*: Replaces a guaranteed heap allocation with a low-cost pointer reference on untransformed lines.
