# Custom Buddy Memory Allocation System

Core **Rust** implementation for **Custom Buddy Memory Allocation System**, structured for computational clarity, explicit data structures, and deterministic unit test coverage.

---

## 🏛️ Architecture & Design Decisions

This module organizes `Custom Buddy Memory Allocation System` into an isolated, self-contained unit:
* **Domain Focus**: `Algorithmic Engineering`
* **Primary Primitives**: `Standard Memory Primitives`
* **Memory Strategy**: Zero external heap dependencies; designed as a pure in-memory algorithmic component.
* **Correctness Model**: State transitions follow clear ordering guarantees with explicit validation at each phase.

### Asymptotic Complexity

| Metric | Bound | Characteristics |
| :--- | :---: | :--- |
| **Best Case Time** | `O(1)` | Optimized fast-path execution |
| **Average / Worst Time** | `O(N)` | Deterministic upper bound for generalized workloads |
| **Space Complexity** | `O(N)` | Strict bounds without unconstrained heap growth |

---

## 🧪 Verification Suite

The accompanying `main.rs` driver executes self-contained verification tests:
1. **Nominal Flow**: Validates baseline correctness under typical real-world inputs.
2. **Boundary Conditions**: Exercises extreme edge cases (empty inputs, singletons, capacity limits).
3. **Invariant Preservation**: Validates internal state consistency throughout mutation lifecycles.

### Running Locally

```bash
rustc -O main.rs -o runner && ./runner
```

---

*Source code released under the MIT License • [@myonathanlinkedin](https://github.com/myonathanlinkedin)*