use crate::types::{VectorClock, ClockRelation};

impl VectorClock {
    /// Creates a new vector clock with `size` entries, all initialised to zero.
    ///
    /// # Panics
    ///
    /// Panics if `size` is zero. A zero‑sized clock is meaningless for ordering.
    pub fn new(size: usize) -> Self {
        assert!(size > 0, "VectorClock size must be greater than zero");
        VectorClock {
            counters: vec![0; size],
        }
    }

    /// Returns the number of processes (i.e., the length of the internal vector).
    #[inline]
    pub fn size(&self) -> usize {
        self.counters.len()
    }

    /// Increments the counter for the process identified by `pid`.
    ///
    /// # Panics
    ///
    /// Panics if `pid` is out of bounds.
    pub fn increment(&mut self, pid: usize) {
        assert!(pid < self.size(), "pid out of bounds");
        self.counters[pid] = self
            .counters
            .get(pid)
            .expect("pid validated")
            .wrapping_add(1);
    }

    /// Merges `other` into `self` by taking the element‑wise maximum.
    ///
    /// # Panics
    ///
    /// Panics if the two clocks have different sizes.
    pub fn merge(&mut self, other: &VectorClock) {
        assert_eq!(
            self.size(),
            other.size(),
            "cannot merge clocks of different sizes"
        );
        for i in 0..self.size() {
            let max = self.counters[i].max(other.counters[i]);
            self.counters[i] = max;
        }
    }

    /// Compares `self` with `other` and returns their causal relationship.
    ///
    /// # Panics
    ///
    /// Panics if the two clocks have different sizes.
    pub fn compare(&self, other: &VectorClock) -> ClockRelation {
        assert_eq!(
            self.size(),
            other.size(),
            "cannot compare clocks of different sizes"
        );

        let mut self_less = false;
        let mut self_greater = false;

        for i in 0..self.size() {
            let a = self.counters[i];
            let b = other.counters[i];
            if a < b {
                self_less = true;
            } else if a > b {
                self_greater = true;
            }
            // Early exit if both directions observed – they are concurrent.
            if self_less && self_greater {
                return ClockRelation::Concurrent;
            }
        }

        match (self_less, self_greater) {
            (false, false) => ClockRelation::Equal,
            (true, false) => ClockRelation::Less,
            (false, true) => ClockRelation::Greater,
            (true, true) => ClockRelation::Concurrent, // unreachable due to early return
        }
    }
}