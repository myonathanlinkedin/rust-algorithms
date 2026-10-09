# Bit-Parallel Levenshtein Distance Matrix Engine in Rust

Self-contained **Bit-Parallel Levenshtein Distance Matrix Engine** algorithmic primitive written in idiomatic **Rust**. Built from scratch using standard library constructs with zero external dependencies.

## Implementation Details

* **Category**: `Computational Mathematics & Transformation`
* **Data Structure Foundation**: `Lookup Tables & Bitwise Bitvectors`
* **Allocation Pattern**: Zero external heap dependencies; designed as a pure in-memory algorithmic component.
* **Invariant Integrity**: State consistency is verified after mutations through assertion test coverage.

## Performance Characteristics

* **Time**: `O(N log N)` average, with `O(N log N)` best-case response under ideal conditions.
* **Space**: `O(N)` memory usage.

## Test Harness

To compile and execute the test assertions for this module:

```bash
rustc -O main.rs -o runner && ./runner
```

---

*Reference implementation verified by [@myonathanlinkedin](https://github.com/myonathanlinkedin)*