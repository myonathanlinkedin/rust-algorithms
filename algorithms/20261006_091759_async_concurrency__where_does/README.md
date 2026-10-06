# Async Concurrency: Where does the scheduler live?

A clean, dependency-free **Rust** reference implementation of **Async Concurrency: Where does the scheduler live?**, focused on core algorithmic mechanics, clear memory layout, and test verification.

### Core Highlights
* **Language & Standard**: Modern `Rust` standard library conventions.
* **Architecture Pattern**: Designed for `Algorithmic Engineering` using `Standard Memory Primitives`.
* **Runtime Overhead**: Memory allocations are kept minimal to maintain clear data locality and predictable memory bounds.
* **Concurrency & Safety**: State consistency is verified after mutations through assertion test coverage.

---

### Complexity Analysis

| Dimension | Bound |
| :--- | :--- |
| **Time (Best Case)** | `$O(1)$` |
| **Time (Worst Case)** | `$O(N \log N)$` |
| **Auxiliary Space** | `$O(N)$` |

---

### Test Suite Execution

Self-contained verification drivers are embedded directly in `main.rs` to validate happy paths, boundary inputs, and invariant preservation.

```bash
rustc -O main.rs -o runner && ./runner
```

---

<sub>Standard Rust reference implementation • Maintained by [@myonathanlinkedin](https://github.com/myonathanlinkedin)</sub>