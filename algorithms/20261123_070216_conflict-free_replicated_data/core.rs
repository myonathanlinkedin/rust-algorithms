use std::collections::HashMap;
use std::cmp;

/// A Positive-Negative Counter (PN-Counter) CRDT.
///
/// Each replica maintains two grow‑only counters:
/// * `increments` records the total increments performed by each replica.
/// * `decrements` records the total decrements performed by each replica.
///
/// The observable value is `sum(increments) - sum(decrements)`.
#[derive(Debug, Clone, PartialEq)]
pub struct PNCounter {
    /// Identifier of the local replica.
    pub replica_id: String,
    /// Grow‑only counter for increments: replica_id → cumulative increment.
    pub increments: HashMap<String, u64>,
    /// Grow‑only counter for decrements: replica_id → cumulative decrement.
    pub decrements: HashMap<String, u64>,

}

impl PNCounter {
    /// Creates a new PN‑Counter for the given replica identifier.
    ///
    /// The internal maps are initialised with a zero entry for the replica
    /// to guarantee deterministic merging.
    pub fn new<S: Into<String>>(replica_id: S) -> Self {
        let id = replica_id.into();
        let mut inc = HashMap::new();
        inc.insert(id.clone(), 0);
        let mut dec = HashMap::new();
        dec.insert(id.clone(), 0);
        PNCounter {
            replica_id: id,
            increments: inc,
            decrements: dec,
        }
    }

    /// Increments the counter by `delta` (must be non‑zero).
    ///
    /// The operation is idempotent when merged because the per‑replica entry
    /// stores the *total* amount ever added.
    pub fn increment(&mut self, delta: u64) {
        if delta == 0 {
            return;
        }
        let entry = self
            .increments
            .entry(self.replica_id.clone())
            .or_insert(0);
        *entry = entry.saturating_add(delta);
    }

    /// Decrements the counter by `delta` (must be non‑zero).
    ///
    /// Mirrors `increment` but records the amount in the `decrements` map.
    pub fn decrement(&mut self, delta: u64) {
        if delta == 0 {
            return;
        }
        let entry = self
            .decrements
            .entry(self.replica_id.clone())
            .or_insert(0);
        *entry = entry.saturating_add(delta);
    }

    /// Returns the current logical value of the counter as a signed integer.
    ///
    /// The result fits in `i64` because each component is a `u64` and the
    /// difference of two `u64`s is bounded by `i64::MAX` for realistic usage.
    pub fn value(&self) -> i64 {
        let inc_sum: u64 = self.increments.values().copied().sum();
        let dec_sum: u64 = self.decrements.values().copied().sum();
        // Cast safely: the difference cannot overflow i64 for typical CRDT workloads.
        inc_sum as i64 - dec_sum as i64
    }

    /// Merges another PN‑Counter into `self`.
    ///
    /// For each replica identifier, the maximum of the two counters is kept.
    /// This operation is commutative, associative, and idempotent.
    pub fn merge(&mut self, other: &PNCounter) {
        // Merge increments
        for (replica, &other_val) in other.increments.iter() {
            let self_val = self.increments.get(replica).copied().unwrap_or(0);
            let merged = cmp::max(self_val, other_val);
            self.increments.insert(replica.clone(), merged);
        }
        // Merge decrements
        for (replica, &other_val) in other.decrements.iter() {
            let self_val = self.decrements.get(replica).copied().unwrap_or(0);
            let merged = cmp::max(self_val, other_val);
            self.decrements.insert(replica.clone(), merged);
        }
    }
}