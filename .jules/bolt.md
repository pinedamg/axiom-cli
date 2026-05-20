# Axiom Memory Discoveries

- **DiscoveryEngine Variable Buffer Overflow**: Previously, the `variable_buffer` tracked all regex-matched variables, storing them in deeply nested vectors (`BTreeMap<String, Vec<Vec<String>>>`). Given Axiom's goal of processing huge log files, this resulted in unbounded heap accumulation per unique log template. To resolve this, `extract_parts` was refactored to skip variable tracking entirely (as they were unused in the final summary) and `variable_buffer` was optimized to just maintain a matching count (`BTreeMap<String, usize>`).

- **Deduplication Hot-path Allocation**: `stage_deduplicate` formerly allocated new memory for `self.discovery.last_line` every time a new line arrived. By employing `Option::take()` and `String::clear()`, we reuse the buffer capacity across the processing pipeline.

- **Pre-allocation Wipe-out Risk**: When dealing with string manipulation at the `StreamFilter` level, replacing `std::mem::take` with `std::mem::replace(..., String::with_capacity(C))` prevents allocating empty-capacity strings, keeping reallocation overhead at 0 for steady streams.
