# ⚡ Bolt: Memory Patterns Discovered

* Track template match counts (`BTreeMap<String, usize>`) instead of storing full regex variable vectors in `DiscoveryEngine`'s `variable_buffer` to drastically reduce memory overhead during heavily templated stream processing.
* Use `String::capacity() + std::mem::size_of::<usize>()` to calculate the footprint for string-to-count map structures correctly.
* Use `.into_owned()` on `Cow<str>` instead of `.to_string()` when working with regex replace operations to avoid unnecessary heap allocations when no replacements are made.
* In high-frequency line-processing hot paths (like `stage_deduplicate`), reuse existing `Option<String>` buffers by extracting them with `.take()`, clearing them, and appending data with `.push_str()` instead of constantly allocating new strings.
* When working with `String` buffers in `StreamPipeline`, use `std::mem::replace(&mut self.buffer, String::with_capacity(1024))` to maintain memory capacity and prevent reallocation overhead when emptying the buffer.
* Prefer `Vec::new()` over `Vec::with_capacity(1)` for collections that are frequently empty (like `plugin_reasons` in `stage_plugins`) to prevent heap allocation for the empty case.
