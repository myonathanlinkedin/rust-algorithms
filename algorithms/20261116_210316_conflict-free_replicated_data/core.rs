use std::collections::HashMap;

/// A Positive-Negative Counter (PN-Counter) CRDT.
///
/// Each replica is identified by a unique `usize` `node_id`.
/// Internally it maintains two grow-only counters:
/// * `p` for increments (positive)
/// * `n` for decrements (negative)
///
/// The counter supports:
/// * `increment(delta)` – add a non‑negative delta
/// * `decrement(delta)` – subtract a non‑negative delta
/// * `value()` – current logical value (sum of all increments minus sum of all decrements)
/// * `merge(other)` – integrate another replica's state (commutative, associative, idempotent)
#[derive(Debug, Clone, PartialEq)]
pub struct PNCounter {
    /// Identifier of this replica.
    pub node_id: usize,
    /// Positive (increment) G‑Counter: replica_id → count.
    pub p: HashMap<usize, u64>,
    /// Negative (decrement) G‑Counter: replica_id → count.
    pub n: HashMap<usize, u64>,

}

impl PNCounter {
    /// Creates a new PN‑Counter for the given replica identifier.
    ///
    /// The internal maps are initialised with a zero entry for `node_id`
    /// to guarantee deterministic merging when a replica has never performed
    /// an operation.
    pub fn new(node_id: usize) -> Self {
        let mut p = HashMap::new();
        let mut n = HashMap::new();
        p.insert(node_id, 0);
        n.insert(node_id, 0);
        Self { node_id, p, n }
    }

    /// Internal helper: fetch mutable reference to a counter map entry,
    /// inserting zero if absent.
    fn entry_mut(map: &mut HashMap<usize, u64>, id: usize) -> &mut u64 {
        map.entry(id).or_insert(0)
    }

    /// Increments the counter by `delta` (must be non‑negative).
    ///
    /// Panics if `delta` would overflow `u64`.
    pub fn increment(&mut self, delta: u64) {
        let entry = Self::entry_mut(&mut self.p, self.node_id);
        *entry = entry
            .checked_add(delta)
            .expect("PNCounter increment overflow");
    }

    /// Decrements the counter by `delta` (must be non‑negative).
    ///
    /// Panics if `delta` would overflow `u64`.
    pub fn decrement(&mut self, delta: u64) {
        let entry = Self::entry_mut(&mut self.n, self.node_id);
        *entry = entry
            .checked_add(delta)
            .expect("PNCounter decrement overflow");
    }

    /// Returns the logical value of the counter:
    ///   Σ increments – Σ decrements
    pub fn value(&self) -> i64 {
        let pos: u64 = self.p.values().copied().sum();
        let neg: u64 = self.n.values().copied().sum();
        // The subtraction cannot underflow because we interpret the result as signed.
        (pos as i128 - neg as i128) as i64
    }

    /// Merges another replica's state into `self`.
    ///
    /// For each replica identifier the maximum observed increment and decrement
    /// counts are retained. This operation is commutative, associative and idempotent.
    pub fn merge(&mut self, other: &Self) {
        // Merge positive counters.
        for (id, &cnt) in other.p.iter() {
            let entry = Self::entry_mut(&mut self.p, *id);
            if cnt > *entry {
                *entry = cnt;
            }
        }
        // Merge negative counters.
        for (id, &cnt) in other.n.iter() {
            let entry = Self::entry_mut(&mut self.n, *id);
            if cnt > *entry {
                *entry = cnt;
            }
        }
    }
}