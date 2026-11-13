use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

/// Compute a deterministic 8‑byte hash of arbitrary data using the standard library
/// `DefaultHasher`. This is **not** cryptographically secure but suffices for a
/// pure‑algorithm reference implementation.
pub fn hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = DefaultHasher::new();
    hasher.write(data);
    let result = hasher.finish();
    result.to_be_bytes().to_vec()
}

/// Hash a pair of child nodes. The left hash is concatenated with the right hash
/// before being fed to the underlying hasher.
pub fn hash_pair(left: &[u8], right: &[u8]) -> Vec<u8> {
    let mut combined = Vec::with_capacity(left.len() + right.len());
    combined.extend_from_slice(left);
    combined.extend_from_slice(right);
    hash(&combined)
}

/// A Merkle tree built from arbitrary leaf data. Internally the tree stores each
/// level of hashes, where `levels[0]` are the leaf hashes and the last level
/// contains a single root hash.
#[derive(Debug, Clone, PartialEq)]
pub struct MerkleTree {
    pub levels: Vec<Vec<Vec<u8>>>,

}

impl MerkleTree {
    /// Construct a new Merkle tree from raw leaf data. Each leaf is hashed
    /// before being placed in the tree. The function panics if `leaves` is empty.
    pub fn new(leaves: Vec<Vec<u8>>) -> Self {
        assert!(!leaves.is_empty(), "Merkle tree must contain at least one leaf");

        // Hash each leaf value.
        let mut current_level: Vec<Vec<u8>> = leaves.iter().map(|d| hash(d)).collect();
        let mut levels = vec![current_level.clone()];

        // Build parent levels until a single root remains.
        while current_level.len() > 1 {
            let mut next_level = Vec::new();
            let mut i = 0usize;
            while i < current_level.len() {
                let left = &current_level[i];
                let right = if i + 1 < current_level.len() {
                    &current_level[i + 1]
                } else {
                    // Duplicate the last node when the count is odd.
                    left
                };
                next_level.push(hash_pair(left, right));
                i += 2;
            }
            levels.push(next_level.clone());
            current_level = next_level;
        }

        MerkleTree { levels }
    }

    /// Return a reference to the root hash of the tree.
    pub fn root(&self) -> &[u8] {
        self.levels.last().unwrap().first().unwrap()
    }

    /// Generate a Merkle proof for the leaf at `leaf_index`. The proof consists
    /// of the sibling hash at each level required to recompute the root.
    pub fn generate_proof(&self, leaf_index: usize) -> MerkleProof {
        assert!(
            leaf_index < self.levels[0].len(),
            "leaf index out of bounds"
        );

        let mut idx = leaf_index;
        let mut sibling_hashes = Vec::new();

        // Iterate over all levels except the root level.
        for level in &self.levels[..self.levels.len() - 1] {
            let sibling_idx = if idx % 2 == 0 { idx + 1 } else { idx - 1 };
            let sibling = if sibling_idx < level.len() {
                level[sibling_idx].clone()
            } else {
                // When the node has no sibling (odd count), duplicate itself.
                level[idx].clone()
            };
            sibling_hashes.push(sibling);
            idx /= 2;
        }

        MerkleProof {
            leaf_index,
            sibling_hashes,
        }
    }
}

/// A Merkle proof consisting of the leaf index and the list of sibling hashes
/// needed to reconstruct the root.
#[derive(Debug, Clone, PartialEq)]
pub struct MerkleProof {
    pub leaf_index: usize,
    pub sibling_hashes: Vec<Vec<u8>>,

}

/// Verify a Merkle proof against a known root hash. Returns `true` iff the proof
/// is valid for the supplied `leaf` data and `leaf_index`.
pub fn verify_proof(root: &[u8], leaf: &[u8], leaf_index: usize, proof: &MerkleProof) -> bool {
    if leaf_index != proof.leaf_index {
        return false;
    }

    // Start with the hash of the raw leaf data.
    let mut computed = hash(leaf);
    let mut idx = leaf_index;

    for sibling_hash in &proof.sibling_hashes {
        if idx % 2 == 0 {
            computed = hash_pair(&computed, sibling_hash);
        } else {
            computed = hash_pair(sibling_hash, &computed);
        }
        idx /= 2;
    }

    computed == root
}