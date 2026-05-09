# Bolt Memory Optimization Journal

* Refactored `ps.rs` to track top process without string cloning. By switching `top_proc` from `String` to `Option<&str>`, redundant allocations inside the loop are avoided.
* Tracked counts instead of vectors in `discovery.rs`. By updating `variable_buffer` from `BTreeMap<String, Vec<Vec<String>>>` to `BTreeMap<String, usize>`, the heavily allocating vector logic is removed during extraction and string grouping. The count is incremented directly and reused for reporting.
* Reused string buffer during deduplication in `mod.rs`. Instead of continually allocating a new string per line in `stage_deduplicate`, the old option value is taken, cleared, pushed to and saved back inside `self.discovery.last_line`.
