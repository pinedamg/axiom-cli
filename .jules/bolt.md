## Memory Footprint Discoveries
* During stream aggregation and parsing in `DiscoveryEngine`, using `HashMap` and large arrays like `Vec<Vec<String>>` can incur massive performance and memory hits since items are allocated recursively per line.
* By switching to `BTreeMap<String, usize>` for string keys such as matching template fragments, we vastly improve our overhead and leverage native sorting without an extra step.

## Memory Footprint Discoveries
* During stream aggregation and parsing in `DiscoveryEngine`, using `HashMap` and large arrays like `Vec<Vec<String>>` can incur massive performance and memory hits since items are allocated recursively per line.
* By switching to `BTreeMap<String, usize>` for string keys such as matching template fragments, we vastly improve our overhead and leverage native sorting without an extra step.

## Memory Footprint Discoveries
* During stream aggregation and parsing in `DiscoveryEngine`, using `HashMap` and large arrays like `Vec<Vec<String>>` can incur massive performance and memory hits since items are allocated recursively per line.
* By switching to `BTreeMap<String, usize>` for string keys such as matching template fragments, we vastly improve our overhead and leverage native sorting without an extra step.
