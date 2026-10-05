# Show HN: Glashütte Trash Clock – A 30-minute pendulum clock made from trash in Rust

High-performance **Show HN: Glashütte Trash Clock – A 30-minute pendulum clock made from trash** primitive implemented in idiomatic **Rust**. Built from scratch using standard library constructs with zero external dependencies.

## Implementation Details

* **Category**: `Low-Latency Systems & Memory Layout`
* **Data Structure Foundation**: `Contiguous Memory Buffer & Ring Pointers`
* **Allocation Pattern**: Contiguous memory layouts are favored over scattered heap allocations for optimal traversal speed.
* **Invariant Integrity**: Designed with reentrancy and thread isolation in mind, preventing data races under parallel workloads.

## Performance Characteristics

* **Time**: `$O(1)$` average, with `$O(1)$` best-case response under ideal conditions.
* **Space**: `$O(N) bounded$` memory usage.

## Test Harness

To compile and execute the test assertions for this module:

```bash
rustc -O main.rs -o runner && ./runner
```

---

*Source code released under the MIT License • [@myonathanlinkedin](https://github.com/myonathanlinkedin)*