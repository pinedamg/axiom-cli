## Memory Consumption Patterns

- Replaced `HashMap` with `BTreeMap` to minimize memory hashing overhead and sort directly.
- The `DiscoveryEngine`'s `variable_buffer` used to store extracted variables as `Vec<Vec<String>>`, leading to high memory overhead when stream processing heavily templated logs. It has been replaced with `usize` template match counts, which significantly reduces deep allocation structures and heap memory usage.
- The `DiscoveryEngine`'s `variable_buffer` used to store extracted variables as `Vec<Vec<String>>`, leading to high memory overhead when stream processing heavily templated logs. It has been replaced with `usize` template match counts, which significantly reduces deep allocation structures and heap memory usage.
