# LSM-Tree MemTable and SSTable Flush Compaction Engine in Rust

A clean, dependency-free **Rust** implementation of **LSM-Tree MemTable and SSTable Flush Compaction Engine**, focused on predictable latency, strict memory layout, and deterministic execution.

## Implementation Details

* **Category**: `Balanced Hierarchical Indexing`
* **Data Structure Foundation**: `Node Pointers & Self-Balancing Trees`
* **Allocation Pattern**: Buffer boundaries are strictly verified to prevent out-of-bounds access and memory leak hazards.
* **Invariant Integrity**: Deterministic behavior across all execution cycles, resilient against asynchronous edge conditions.

## Performance Characteristics

* **Time**: `$O(\log N)$` average, with `$O(1)$` best-case response under ideal conditions.
* **Space**: `$O(N)$` memory usage.

## Test Harness

To compile and execute the test assertions for this module:

```bash
rustc -O main.rs -o runner && ./runner
```

---

*Curated as part of the Polyglot Systems Lab • Maintained by [@myonathanlinkedin](https://github.com/myonathanlinkedin)*