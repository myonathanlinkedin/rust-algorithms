use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

const BUCKET_SIZE: usize = 4;
const DEFAULT_MAX_KICKS: usize = 500;

#[derive(Debug, Clone, PartialEq)]
pub struct CuckooFilter {
    pub buckets: Vec<[Option<u8>; BUCKET_SIZE]>,
    pub bucket_count: usize,
    pub max_kicks: usize,

}

impl CuckooFilter {
    /// Create a new cuckoo filter with at least `capacity` items.
    /// The actual number of buckets will be the next power of two
    /// greater than or equal to `capacity / BUCKET_SIZE`.
    pub fn new(capacity: usize) -> Self {
        let bucket_count = (capacity / BUCKET_SIZE).next_power_of_two().max(1);
        let buckets = vec![[None; BUCKET_SIZE]; bucket_count];
        Self {
            buckets,
            bucket_count,
            max_kicks: DEFAULT_MAX_KICKS,
        }
    }

    /// Insert an item. Returns `true` on success, `false` if the filter is full.
    pub fn insert<T: Hash>(&mut self, item: &T) -> bool {
        let (i1, i2, fp) = self.compute_indices_and_fp(item);
        if self.insert_into_bucket(i1, fp) || self.insert_into_bucket(i2, fp) {
            return true;
        }

        let mut cur_fp = fp;
        let mut cur_idx = i1;
        for _ in 0..self.max_kicks {
            // Evict a fingerprint from the current bucket
            let evict_pos = self.evict_random(cur_idx);
            let evicted_fp = self.buckets[cur_idx][evict_pos].replace(cur_fp).unwrap();
            cur_fp = evicted_fp;
            // Compute alternate index for the evicted fingerprint
            cur_idx = self.alternate_index(cur_idx, cur_fp);
            if self.insert_into_bucket(cur_idx, cur_fp) {
                return true;
            }
        }
        false // filter is considered full
    }

    /// Check whether an item is possibly in the filter.
    pub fn contains<T: Hash>(&self, item: &T) -> bool {
        let (i1, i2, fp) = self.compute_indices_and_fp(item);
        self.bucket_contains(i1, fp) || self.bucket_contains(i2, fp)
    }

    /// Delete an item. Returns `true` if the item was found and removed.
    pub fn delete<T: Hash>(&mut self, item: &T) -> bool {
        let (i1, i2, fp) = self.compute_indices_and_fp(item);
        if self.remove_from_bucket(i1, fp) || self.remove_from_bucket(i2, fp) {
            return true;
        }
        false
    }

    // ---------- internal helpers ----------
    fn compute_indices_and_fp<T: Hash>(&self, item: &T) -> (usize, usize, u8) {
        let hash = hash(item);
        let fp = fingerprint(hash);
        let i1 = (hash as usize) % self.bucket_count;
        let i2 = self.alternate_index(i1, fp);
        (i1, i2, fp)
    }

    fn alternate_index(&self, idx: usize, fp: u8) -> usize {
        let fp_hash = hash(&fp);
        ((idx ^ (fp_hash as usize)) % self.bucket_count) as usize
    }

    fn insert_into_bucket(&mut self, idx: usize, fp: u8) -> bool {
        for slot in self.buckets[idx].iter_mut() {
            if slot.is_none() {
                *slot = Some(fp);
                return true;
            }
        }
        false
    }

    fn bucket_contains(&self, idx: usize, fp: u8) -> bool {
        self.buckets[idx].iter().any(|&s| s == Some(fp))
    }

    fn remove_from_bucket(&mut self, idx: usize, fp: u8) -> bool {
        for slot in self.buckets[idx].iter_mut() {
            if *slot == Some(fp) {
                *slot = None;
                return true;
            }
        }
        false
    }

    fn evict_random(&self, idx: usize) -> usize {
        // Deterministic choice: first occupied slot; if none (should not happen), 0.
        for (pos, slot) in self.buckets[idx].iter().enumerate() {
            if slot.is_some() {
                return pos;
            }
        }
        0
    }
}

// Simple 64‑bit hash using the default hasher.
pub fn hash<T: Hash>(item: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    item.hash(&mut hasher);
    hasher.finish()
}

// 8‑bit fingerprint; zero is avoided because it would be indistinguishable from empty.
pub fn fingerprint(hash: u64) -> u8 {
    let mut fp = (hash & 0xFF) as u8;
    if fp == 0 {
        fp = 1;
    }
    fp
}