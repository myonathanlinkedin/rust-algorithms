pub mod core {
    #[derive(Debug, Clone, PartialEq)]
    pub struct VectorClock {
        pub counters: Vec<u64>,

    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum ClockRelation {
        Less,
        Equal,
        Greater,
        Concurrent,
    }

    impl VectorClock {
        /// Create a new vector clock for `size` processes, all counters start at 0.
        pub fn new(size: usize) -> Self {
            Self {
                counters: vec![0; size],
            }
        }

        /// Number of processes tracked by this clock.
        pub fn size(&self) -> usize {
            self.counters.len()
        }

        /// Increment the counter for process `pid`.
        pub fn increment(&mut self, pid: usize) {
            assert!(pid < self.size(), "pid out of bounds");
            // Wrapping add protects against overflow in pathological tests.
            self.counters[pid] = self.counters[pid].wrapping_add(1);
        }

        /// Set the counter for process `pid` to `value`.
        pub fn set(&mut self, pid: usize, value: u64) {
            assert!(pid < self.size(), "pid out of bounds");
            self.counters[pid] = value;
        }

        /// Retrieve the counter for process `pid`.
        pub fn get(&self, pid: usize) -> u64 {
            assert!(pid < self.size(), "pid out of bounds");
            self.counters[pid]
        }

        /// Merge another clock into this one by taking the element‑wise maximum.
        pub fn merge(&mut self, other: &Self) {
            assert_eq!(self.size(), other.size(), "size mismatch");
            for i in 0..self.size() {
                if other.counters[i] > self.counters[i] {
                    self.counters[i] = other.counters[i];
                }
            }
        }

        /// Compare two vector clocks.
        ///
        /// Returns:
        /// - `Equal` if all entries are identical.
        /// - `Less` if `self` is causally before `other`.
        /// - `Greater` if `self` is causally after `other`.
        /// - `Concurrent` otherwise.
        pub fn compare(&self, other: &Self) -> ClockRelation {
            assert_eq!(self.size(), other.size(), "size mismatch");
            let mut self_leq = true;
            let mut other_leq = true;
            for i in 0..self.size() {
                let a = self.counters[i];
                let b = other.counters[i];
                if a < b {
                    self_leq = false;
                }
                if a > b {
                    other_leq = false;
                }
            }
            match (self_leq, other_leq) {
                (true, true) => ClockRelation::Equal,
                (true, false) => ClockRelation::Less,
                (false, true) => ClockRelation::Greater,
                (false, false) => ClockRelation::Concurrent,
            }
        }
    }
}

// Re‑export for convenient external use.
pub use core::{ClockRelation, VectorClock};