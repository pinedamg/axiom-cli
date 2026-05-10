# Bolt Memory Optimizations

Discovered memory consumption pattern specific to Axiom's architecture:
During stream processing, heavily templated logs cause massive heap memory allocation overhead because the `DiscoveryEngine` stores full vectors of extracted regex variables inside `variable_buffer`. Changing this to track only match counts (`usize`) drastically reduces memory footprint.

Also:
- Reused `Option<String>` buffers in `stage_deduplicate` to avoid constant re-allocation of strings.
- Passed `LineMetadata` by reference instead of `String` clones in `CommandHandler::get_category`.
- Reused reference `Option<&str>` instead of `.clone()` for `top_proc` in `PsHandler::generate_insight`.
