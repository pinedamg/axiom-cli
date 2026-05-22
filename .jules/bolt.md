## Memory Pattern Discoveries

* **DiscoveryEngine Variable Buffer Memory Overhead:**
  The `variable_buffer` in `src/engine/discovery.rs` was storing `Vec<Vec<String>>` for every match. For a large number of lines with many variables, this resulted in massive memory accumulation, especially when lines matched the same template repeatedly (e.g., `Container <UUID> started. Image: <HEX>`). We optimized this by only keeping the count `usize` of matches for each template (as we just needed the template and its count, not every single extracted substring), saving significant heap allocations.


* **DiscoveryEngine Variable Buffer Memory Overhead:**
  The `variable_buffer` in `src/engine/discovery.rs` was storing `Vec<Vec<String>>` for every match. For a large number of lines with many variables, this resulted in massive memory accumulation, especially when lines matched the same template repeatedly (e.g., `Container <UUID> started. Image: <HEX>`). We optimized this by only keeping the count `usize` of matches for each template (as we just needed the template and its count, not every single extracted substring), saving significant heap allocations.
