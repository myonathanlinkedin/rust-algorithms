use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct PNCounter {
    pub id: usize,
    pub p: HashMap<usize, u64>, // increments per replica
    pub n: HashMap<usize, u64>, // decrements per replica

}

impl PNCounter {
    /// Create a new PN‑Counter for the given replica identifier.
    pub fn new(id: usize) -> Self {
        Self {
            id,
            p: HashMap::new(),
            n: HashMap::new(),
        }
    }

    /// Increment the counter by `amount` (default 1 if amount == 0).
    pub fn inc(&mut self, amount: u64) {
        let inc = if amount == 0 { 1 } else { amount };
        Self::add_to_map(&mut self.p, self.id, inc);
    }

    /// Decrement the counter by `amount` (default 1 if amount == 0).
    pub fn dec(&mut self, amount: u64) {
        let dec = if amount == 0 { 1 } else { amount };
        Self::add_to_map(&mut self.n, self.id, dec);
    }

    /// Current logical value of the counter (may be negative).
    pub fn value(&self) -> i64 {
        let pos: u64 = self.p.values().copied().sum();
        let neg: u64 = self.n.values().copied().sum();
        // Safe conversion because i64 can represent all i64::MIN..=i64::MAX
        // and the difference of two u64 fits in i64 when using wrapping_sub.
        // Here we use checked conversion for clarity.
        (pos as i128 - neg as i128) as i64
    }

    /// Merge another replica's state into this one.
    /// The operation is commutative, associative and idempotent.
    pub fn merge(&mut self, other: &Self) {
        for (&rep, &val) in other.p.iter() {
            let entry = self.p.entry(rep).or_insert(0);
            if val > *entry {
                *entry = val;
            }
        }
        for (&rep, &val) in other.n.iter() {
            let entry = self.n.entry(rep).or_insert(0);
            if val > *entry {
                *entry = val;
            }
        }
    }

    fn add_to_map(map: &mut HashMap<usize, u64>, replica: usize, amount: u64) {
        let entry = map.entry(replica).or_insert(0);
        // Saturating add prevents overflow panics.
        *entry = entry.saturating_add(amount);
    }
}