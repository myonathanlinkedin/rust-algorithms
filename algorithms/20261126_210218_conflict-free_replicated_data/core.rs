use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PNCounter {
    pub counters: HashMap<usize, (u64, u64)>,

}

impl PNCounter {
    pub fn new() -> Self {
        Self {
            counters: HashMap::new(),
        }
    }

    pub fn inc(&mut self, node: usize) {
        let entry = self.counters.entry(node).or_insert((0, 0));
        entry.0 += 1;
    }

    pub fn dec(&mut self, node: usize) {
        let entry = self.counters.entry(node).or_insert((0, 0));
        entry.1 += 1;
    }

    pub fn value(&self) -> i64 {
        let mut sum_p: u64 = 0;
        let mut sum_n: u64 = 0;
        for (_, (p, n)) in &self.counters {
            sum_p += *p;
            sum_n += *n;
        }
        sum_p as i64 - sum_n as i64
    }

    pub fn merge(&mut self, other: &Self) {
        for (&node, (p_other, n_other)) in &other.counters {
            let entry = self.counters.entry(node).or_insert((0, 0));
            if *p_other > entry.0 {
                entry.0 = *p_other;
            }
            if *n_other > entry.1 {
                entry.1 = *n_other;
            }
        }
    }
}