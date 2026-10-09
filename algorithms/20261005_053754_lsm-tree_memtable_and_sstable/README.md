# LSM-Tree MemTable and SSTable Flush Compaction Engine in Rust

A clean, dependency-free **Rust** reference implementation of **LSM-Tree MemTable and SSTable Flush Compaction Engine**, focused on core algorithmic mechanics, clear memory layout, and test verification.

## Implementation Details

* **Category**: `Balanced Hierarchical Indexing`
* **Data Structure Foundation**: `Node Pointers & Self-Balancing Trees`
* **Allocation Pattern**: Buffer boundaries and collection indices are explicitly validated to prevent out-of-bounds access.
* **Invariant Integrity**: Execution behavior is validated against nominal workflows and boundary edge cases.

## Performance Characteristics

* **Time**: `O(log N)` average, with `O(1)` best-case response under ideal conditions.
* **Space**: `O(N)` memory usage.

## Test Harness

To compile and execute the test assertions for this module:

```bash
rustc -O main.rs -o runner && ./runner
```

---

*Part of the Polyglot Systems Lab • Maintained by [@myonathanlinkedin](https://github.com/myonathanlinkedin)*
