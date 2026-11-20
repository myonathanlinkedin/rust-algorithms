use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash as StdHash, Hasher};

pub type Hash = u64;

/// Compute a deterministic hash of a byte slice using the standard library's `DefaultHasher`.
pub fn hash_bytes(data: &[u8]) -> Hash {
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    hasher.finish()
}

/// Combine two child hashes into a parent hash. The order matters.
pub fn hash_pair(left: Hash, right: Hash) -> Hash {
    let mut hasher = DefaultHasher::new();
    left.hash(&mut hasher);
    right.hash(&mut hasher);
    hasher.finish()
}

/// A Merkle tree built over arbitrary byte slices.
///
/// The tree stores all levels internally to allow O(log n) proof generation.
#[derive(Debug, Clone, PartialEq)]
pub struct MerkleTree {
    /// Levels of the tree, where `levels[0]` are leaf hashes,
    /// and the last element contains a single root hash.
    pub levels: Vec<Vec<Hash>>,

}

impl MerkleTree {
    /// Construct a Merkle tree from a slice of data blocks.
    ///
    /// Each block is hashed individually to form the leaf layer.
    /// If the number of leaves is not a power of two, the last leaf
    /// is duplicated until a full binary tree can be formed.
    pub fn new(data_blocks: &[Vec<u8>]) -> Self {
        // Edge case: empty input yields a tree with a single zero hash as root.
        if data_blocks.is_empty() {
            return MerkleTree {
                levels: vec![vec![0]],
            };
        }

        // Compute leaf hashes.
        let mut leaves: Vec<Hash> = data_blocks.iter().map(|b| hash_bytes(b)).collect();

        // Pad to next power of two by duplicating the last leaf.
        let mut size = leaves.len();
        if !size.is_power_of_two() {
            let next_pow = size.next_power_of_two();
            let last = *leaves.last().unwrap();
            leaves.extend(std::iter::repeat(last).take(next_pow - size));
            size = next_pow;
        }

        let mut levels = Vec::new();
        levels.push(leaves.clone());

        // Build upper levels.
        let mut current = leaves;
        while current.len() > 1 {
            let mut next = Vec::with_capacity(current.len() / 2);
            for i in (0..current.len()).step_by(2) {
                let parent = hash_pair(current[i], current[i + 1]);
                next.push(parent);
            }
            levels.push(next.clone());
            current = next;
        }

        MerkleTree { levels }
    }

    /// Return the root hash of the tree.
    pub fn root(&self) -> Hash {
        // The last level always contains exactly one element.
        *self.levels.last().unwrap().first().unwrap()
    }

    /// Generate a Merkle proof for the leaf at `leaf_index`.
    ///
    /// The proof is a vector of `(sibling_hash, sibling_is_left)` tuples.
    /// `sibling_is_left` is `true` when the sibling node is the left child
    /// of the parent, i.e., the current node is the right child.
    pub fn gen_proof(&self, leaf_index: usize) -> Vec<(Hash, bool)> {
        assert!(leaf_index < self.leaf_count(), "leaf index out of bounds");
        let mut proof = Vec::new();
        let mut idx = leaf_index;

        for level in 0..self.levels.len() - 1 {
            let sibling_idx = if idx % 2 == 0 { idx + 1 } else { idx - 1 };
            let sibling_hash = self.levels[level][sibling_idx];
            let sibling_is_left = sibling_idx % 2 == 0;
            proof.push((sibling_hash, sibling_is_left));
            idx /= 2; // Move to parent index.
        }

        proof
    }

    /// Verify a Merkle proof against a given root hash.
    ///
    /// Returns `true` if the proof is valid for `data_block`.
    pub fn verify_proof(data_block: &[u8], proof: &[(Hash, bool)], expected_root: Hash) -> bool {
        let mut cur_hash = hash_bytes(data_block);
        for &(sibling_hash, sibling_is_left) in proof {
            cur_hash = if sibling_is_left {
                hash_pair(sibling_hash, cur_hash)
            } else {
                hash_pair(cur_hash, sibling_hash)
            };
        }
        cur_hash == expected_root
    }

    /// Number of leaves (including any padding duplicates).
    pub fn leaf_count(&self) -> usize {
        self.levels.first().map_or(0, |v| v.len())
    }
}