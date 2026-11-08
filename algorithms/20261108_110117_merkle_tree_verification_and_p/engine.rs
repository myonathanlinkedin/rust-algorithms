use crate::types::{Hash, MerkleTree, MerkleProof, Direction};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hasher, Hash as StdHash};

/// Simple deterministic hash using the standard library's DefaultHasher.
/// The resulting 64‑bit hash is expanded to 32 bytes by repeating its byte pattern.
pub fn hash(data: &[u8]) -> Hash {
    let mut hasher = DefaultHasher::new();
    hasher.write(data);
    let h64 = hasher.finish();
    let mut out = [0u8; 32];
    for i in 0..4 {
        out[i * 8..(i + 1) * 8].copy_from_slice(&h64.to_be_bytes());
    }
    out
}

/// Build a Merkle tree from a slice of leaf data blocks.
/// Each leaf is the hash of the corresponding data block.
pub fn build_tree(leaves: &[Vec<u8>]) -> MerkleTree {
    // Compute leaf hashes
    let mut current_level: Vec<Hash> = leaves.iter().map(|d| hash(d)).collect();
    let mut levels = Vec::new();
    levels.push(current_level.clone());

    // Build upper levels until a single root remains
    while current_level.len() > 1 {
        let mut next_level = Vec::new();
        let mut i = 0usize;
        while i < current_level.len() {
            let left = current_level[i];
            let right = if i + 1 < current_level.len() {
                current_level[i + 1]
            } else {
                // Duplicate last hash if odd number of nodes
                current_level[i]
            };
            // Concatenate left and right hashes and hash the result
            let mut combined = Vec::with_capacity(64);
            combined.extend_from_slice(&left);
            combined.extend_from_slice(&right);
            next_level.push(hash(&combined));
            i += 2;
        }
        levels.push(next_level.clone());
        current_level = next_level;
    }

    MerkleTree { levels }
}

/// Return the root hash of the tree.
pub fn root(tree: &MerkleTree) -> Hash {
    // The last level always contains exactly one hash (the root)
    tree.levels.last().unwrap()[0]
}

/// Generate a Merkle proof for the leaf at `leaf_index`.
pub fn generate_proof(tree: &MerkleTree, leaf_index: usize) -> MerkleProof {
    assert!(!tree.levels.is_empty(), "Tree must have at least one level");
    assert!(leaf_index < tree.levels[0].len(), "Leaf index out of bounds");

    let mut idx = leaf_index;
    let mut sibling_hashes = Vec::new();

    for level in 0..tree.levels.len() - 1 {
        let level_hashes = &tree.levels[level];
        let is_left = idx % 2 == 0;
        let sibling_idx = if is_left {
            if idx + 1 < level_hashes.len() {
                idx + 1
            } else {
                // No sibling (odd count), sibling is the node itself
                idx
            }
        } else {
            idx - 1
        };
        let sibling_hash = level_hashes[sibling_idx];
        let direction = if is_left { Direction::Right } else { Direction::Left };
        sibling_hashes.push((direction, sibling_hash));
        // Move to parent index for next level
        idx /= 2;
    }

    MerkleProof {
        leaf_index,
        sibling_hashes,
        root: root(tree),
    }
}

/// Verify a Merkle proof against a leaf data block.
/// Returns true iff the proof reconstructs the stored root.
pub fn verify_proof(proof: &MerkleProof, leaf_data: &Vec<u8>) -> bool {
    // Start from leaf hash
    let mut computed_hash = hash(leaf_data);
    let mut idx = proof.leaf_index;

    for (direction, sibling_hash) in &proof.sibling_hashes {
        let (left, right) = match direction {
            Direction::Left => (sibling_hash.clone(), computed_hash),
            Direction::Right => (computed_hash, sibling_hash.clone()),
        };
        let mut combined = Vec::with_capacity(64);
        combined.extend_from_slice(&left);
        combined.extend_from_slice(&right);
        computed_hash = hash(&combined);
        idx /= 2;
    }

    computed_hash == proof.root
}