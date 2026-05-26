# Discovery Engine Memory Optimizations

## Buffer Refactoring
We identified that `variable_buffer` was previously declared as `BTreeMap<String, Vec<Vec<String>>>` to track regex template variables. However, the exact strings in those vectors were never actually utilized, only the length of the lists (`var_sets.len()`) to determine repeat counts.

We refactored `variable_buffer` to `BTreeMap<String, usize>`, strictly storing the repetition count.

This significantly drops heap memory usage, since we no longer repeatedly allocate and push to dynamic vectors of strings for every matched line.

## String Extraction & Capacity Reuse
1. `extract_parts` was refactored to directly perform inline regex string replacements and return a single `String`, completely avoiding `Vec<String>` allocations for regex captures.
2. In `get_saved_bytes`, we refactored the estimate calculations to `template.capacity() + std::mem::size_of::<usize>()` to prevent multiplying the raw memory cost by match counts incorrectly.
3. In `mod.rs` `stage_deduplicate`, we utilized `take().unwrap_or_default()`, `clear()`, and `push_str()` instead of allocating fresh `String` elements for the stream pipeline `last_line` variable.
4. In `gateway/filters.rs`, we swapped out `std::mem::take(&mut self.buffer)` for `self.buffer.clone()` + `self.buffer.clear()` to maintain the underlying 1024-byte capacity instead of reallocating it constantly.
