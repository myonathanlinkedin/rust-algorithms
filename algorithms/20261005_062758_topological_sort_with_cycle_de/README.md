# Topological Sort with Cycle Detection in Directed Graphs

A clean, dependency-free **Rust** reference implementation of **Topological Sort with Cycle Detection in Directed Graphs**, focused on core algorithmic mechanics, clear memory layout, and test verification.

### Core Highlights
* **Language & Standard**: Modern `Rust` standard library conventions.
* **Architecture Pattern**: Designed for `Graph Topology & Traversal` using `Adjacency List & Priority Heap`.
* **Runtime Overhead**: Contiguous memory layouts and standard collections are favored for straightforward iteration and access.
* **Concurrency & Safety**: State consistency is verified after mutations through assertion test coverage.

---

### Complexity Analysis

| Dimension | Bound |
| :--- | :--- |
| **Time (Best Case)** | `O(V + E)` |
| **Time (Worst Case)** | `O(V^2)` |
| **Auxiliary Space** | `O(V + E)` |

---

### Test Suite Execution

Self-contained verification drivers are embedded directly in `main.rs` to validate happy paths, boundary inputs, and invariant preservation.

```bash
rustc -O main.rs -o runner && ./runner
```

---

*Part of the Polyglot Systems Lab • Maintained by [@myonathanlinkedin](https://github.com/myonathanlinkedin)*
