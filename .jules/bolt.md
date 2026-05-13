## Memory Footprint of `BTreeMap<String, usize>` Structures

When estimating the memory footprint of `BTreeMap<String, usize>` structures in Axiom (e.g., within `DiscoveryEngine::get_saved_bytes`), calculate the actual memory consumed using the string key's capacity and the value's size (`template.capacity() + std::mem::size_of::<usize>()`) rather than multiplying string lengths by match counts.

## Re-using Option<String> in Hot Paths

In high-frequency line-processing hot paths (e.g., `stage_deduplicate`), reuse existing `Option<String>` buffers by extracting them with `.take()`, clearing with `.clear()`, and appending data with `.push_str()` instead of constantly allocating new strings on the heap.
