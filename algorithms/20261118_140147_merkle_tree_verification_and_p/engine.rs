use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash as StdHash, Hasher};

use crate::types::{Hash, Proof};

pub fn hash_bytes(data: &[u8]) -> Hash {
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    hasher.finish()
}

pub fn hash_pair(left: Hash, right: Hash) -> Hash {
    let mut hasher = DefaultHasher::new();
    left.hash(&mut hasher);
    right.hash(&mut hasher);
    hasher.finish()
}

#[derive(Debug, Clone, PartialEq)]
pub struct MerkleTree {
    // levels[0] = leaf hashes, levels[1] = parent hashes, ..., levels.last() = root (single element)
    pub levels: Vec<Vec<Hash>>,
    pub leaf_count: usize,

}

impl MerkleTree {
    pub fn new(leaves: Vec<Vec<u8>>) -> Self {
        let leaf_hashes: Vec<Hash> = leaves.iter().map(|d| hash_bytes(d)).collect();
        let leaf_count = leaf_hashes.len();
        let mut levels = Vec::new();
        levels.push(leaf_hashes);
        // Build upper levels
        while levels.last().unwrap().len() > 1 {
            let prev = levels.last().unwrap();
            let mut next = Vec::new();
            let mut i = 0usize;
            while i < prev.len() {
                let left = prev[i];
                let right = if i + 1 < prev.len() {
                    prev[i + 1]
                } else {
                    // duplicate last hash when odd number of nodes
                    prev[i]
                };
                next.push(hash_pair(left, right));
                i += 2;
            }
            levels.push(next);
        }
        MerkleTree { levels, leaf_count }
    }

    pub fn root(&self) -> Hash {
        // If tree is empty, define root as 0
        if self.levels.is_empty() {
            0
        } else {
            *self.levels.last().unwrap().first().unwrap()
        }
    }

    pub fn leaf_count(&self) -> usize {
        self.leaf_count
    }

    pub fn generate_proof(&self, leaf_index: usize) -> Option<Proof> {
        if leaf_index >= self.leaf_count {
            return None;
        }
        let mut idx = leaf_index;
        let mut siblings = Vec::new();

        for level in &self.levels {
            if level.len() == 1 {
                break; // reached root
            }
            let sibling_idx = if idx % 2 == 0 { idx + 1 } else { idx - 1 };
            let sibling_hash = if sibling_idx < level.len() {
                level[sibling_idx]
            } else {
                // duplicate when out of bounds (odd count)
                level[idx]
            };
            let is_left = sibling_idx < idx;
            siblings.push((sibling_hash, is_left));
            idx /= 2; // move to parent index for next level
        }

        Some(Proof { siblings })
    }

    pub fn verify_proof(
        root: Hash,
        leaf_data: &[u8],
        leaf_index: usize,
        proof: &Proof,
        total_leaves: usize,
    ) -> bool {
        if leaf_index >= total_leaves {
            return false;
        }
        let mut computed_hash = hash_bytes(leaf_data);
        let mut idx = leaf_index;

        for &(sibling_hash, is_left) in &proof.siblings {
            if is_left {
                computed_hash = hash_pair(sibling_hash, computed_hash);
            } else {
                computed_hash = hash_pair(computed_hash, sibling_hash);
            }
            idx /= 2;
        }

        // After processing all siblings, we should be at the root level.
        // For a well‑formed proof, the number of steps equals the height of the tree.
        // The caller supplies the expected root; we just compare.
        computed_hash == root
    }
}