# Kruskal Minimum Spanning Tree with Disjoint-Set Union in Rust

A clean, dependency-free **Rust** reference implementation of **Kruskal Minimum Spanning Tree with Disjoint-Set Union**, focused on core algorithmic mechanics, clear memory layout, and test verification.

## Implementation Details

* **Category**: `Balanced Hierarchical Indexing`
* **Data Structure Foundation**: `Node Pointers & Self-Balancing Trees`
* **Allocation Pattern**: Contiguous memory layouts and standard collections are favored for straightforward iteration and access.
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

<sub>Standard Rust reference implementation • Maintained by [@myonathanlinkedin](https://github.com/myonathanlinkedin)</sub>