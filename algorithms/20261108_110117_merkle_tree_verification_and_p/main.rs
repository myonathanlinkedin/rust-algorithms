mod types;
mod engine;

use crate::types::{MerkleTree, MerkleProof};
use crate::engine::{build_tree, generate_proof, verify_proof, root};

fn main() {
    // Simple deterministic data blocks
    let data_blocks: Vec<Vec<u8>> = vec![
        b"block0".to_vec(),
        b"block1".to_vec(),
        b"block2".to_vec(),
        b"block3".to_vec(),
        b"block4".to_vec(),
    ];

    // Build Merkle tree
    let tree = build_tree(&data_blocks);
    let tree_root = root(&tree);
    println!("Merkle root: {:x?}", tree_root);

    // Test each leaf's proof
    for (i, leaf) in data_blocks.iter().enumerate() {
        let proof = generate_proof(&tree, i);
        assert_eq!(proof.leaf_index, i);
        assert_eq!(proof.root, tree_root);
        let ok = verify_proof(&proof, leaf);
        assert!(ok, "Proof verification failed for leaf {}", i);
    }

    // Edge case: single leaf
    let single = vec![b"only".to_vec()];
    let single_tree = build_tree(&single);
    let single_root = root(&single_tree);
    let single_proof = generate_proof(&single_tree, 0);
    assert_eq!(single_proof.sibling_hashes.len(), 0);
    assert!(verify_proof(&single_proof, &single[0]));

    // Edge case: odd number of leaves (already covered above)
    // Additional sanity check: tampered data should fail verification
    let mut bad_leaf = data_blocks[0].clone();
    bad_leaf[0] ^= 0xFF; // flip a bit
    let proof0 = generate_proof(&tree, 0);
    assert!(!verify_proof(&proof0, &bad_leaf), "Tampered leaf incorrectly verified");

    println!("All Merkle tree tests passed.");
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::{hash};

    #[test]
    fn test_hash_determinism() {
        let a = b"hello";
        let b = b"hello";
        let c = b"world";
        assert_eq!(hash(a), hash(b));
        assert_ne!(hash(a), hash(c));
    }

    #[test]
    fn test_tree_construction_and_root() {
        let data = vec![b"a".to_vec(), b"b".to_vec(), b"c".to_vec()];
        let tree = build_tree(&data);
        // Root should be deterministic; recompute manually for sanity
        let leaf_hashes: Vec<_> = data.iter().map(|d| hash(d)).collect();
        // Level 1
        let mut level1 = Vec::new();
        for i in (0..leaf_hashes.len()).step_by(2) {
            let left = leaf_hashes[i];
            let right = if i + 1 < leaf_hashes.len() { leaf_hashes[i + 1] } else { leaf_hashes[i] };
            let mut combined = Vec::with_capacity(64);
            combined.extend_from_slice(&left);
            combined.extend_from_slice(&right);
            level1.push(hash(&combined));
        }
        // Level 2 (root)
        let mut combined = Vec::with_capacity(64);
        combined.extend_from_slice(&level1[0]);
        combined.extend_from_slice(&level1[1 % level1.len()]);
        let expected_root = hash(&combined);
        assert_eq!(root(&tree), expected_root);
    }

    #[test]
    fn test_proof_verification() {
        let data = vec![
            b"alpha".to_vec(),
            b"beta".to_vec(),
            b"gamma".to_vec(),
            b"delta".to_vec(),
        ];
        let tree = build_tree(&data);
        for i in 0..data.len() {
            let proof = generate_proof(&tree, i);
            assert!(verify_proof(&proof, &data[i]));
        }
    }

    #[test]
    fn test_invalid_proof() {
        let data = vec![b"x".to_vec(), b"y".to_vec()];
        let tree = build_tree(&data);
        let proof = generate_proof(&tree, 0);
        let mut bad = data[0].clone();
        bad[0] ^= 0xAA;
        assert!(!verify_proof(&proof, &bad));
    }
}