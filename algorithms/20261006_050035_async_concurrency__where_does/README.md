# Async Concurrency: Where does the scheduler live?

A clean, dependency-free **Rust** implementation of **Async Concurrency: Where does the scheduler live?**, focused on predictable latency, strict memory layout, and deterministic execution.

### Core Highlights
* **Language & Standard**: Modern `Rust` standard library conventions.
* **Architecture Pattern**: Designed for `Algorithmic Engineering` using `Standard Memory Primitives`.
* **Runtime Overhead**: Memory allocations are kept minimal to avoid allocator contention and preserve CPU cache locality.
* **Concurrency & Safety**: State consistency is verified after every mutation through formal invariant validation.

---

### Complexity Analysis

| Dimension | Bound |
| :--- | :--- |
| **Time (Best Case)** | `$O(1)$` |
| **Time (Worst Case)** | `$O(N \log N)$` |
| **Auxiliary Space** | `$O(N)$` |

---

### Test Suite Execution

Self-contained verification drivers are embedded directly in `types.rs` to validate happy paths, boundary inputs, and invariant preservation.

```bash
rustc -O types.rs -o runner && ./runner
```

---

<sub>Crafted with modern Rust standards • Maintained by [@myonathanlinkedin](https://github.com/myonathanlinkedin)</sub>