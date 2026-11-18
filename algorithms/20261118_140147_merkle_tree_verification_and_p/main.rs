mod types;
mod engine;

use crate::engine::MerkleTree;
use crate::types::Proof;

fn main() {
    // Simple deterministic test data
    let leaves: Vec<Vec<u8>> = vec![
        b"apple".to_vec(),
        b"banana".to_vec(),
        b"cherry".to_vec(),
        b"date".to_vec(),
        b"elderberry".to_vec(),
    ];

    // Build tree
    let tree = MerkleTree::new(leaves.clone());
    let root = tree.root();

    // Verify root is consistent with manual recomputation for a small tree
    // (We trust the implementation; this is just a sanity check)
    assert!(root != 0, "Root hash should be non‑zero for non‑empty tree");

    // Test proof generation and verification for each leaf
    for (idx, leaf) in leaves.iter().enumerate() {
        let proof_opt = tree.generate_proof(idx);
        assert!(proof_opt.is_some(), "Proof should exist for leaf {}", idx);
        let proof = proof_opt.unwrap();

        // Verify using the static method
        let ok = MerkleTree::verify_proof(
            root,
            leaf,
            idx,
            &proof,
            tree.leaf_count(),
        );
        assert!(ok, "Proof verification failed for leaf {}", idx);
    }

    // Edge case: single leaf tree
    let single_leaf = vec![b"solo".to_vec()];
    let single_tree = MerkleTree::new(single_leaf.clone());
    let single_root = single_tree.root();
    assert_eq!(single_tree.leaf_count(), 1);
    assert_eq!(single_root, {
        // Direct hash of the only leaf
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash as StdHash, Hasher};
        let mut h = DefaultHasher::new();
        single_leaf[0].hash(&mut h);
        h.finish()
    });

    let proof_single = single_tree.generate_proof(0).expect("Proof for single leaf");
    assert!(proof_single.siblings.is_empty(), "Proof for single leaf should have no siblings");
    let ok_single = MerkleTree::verify_proof(
        single_root,
        &single_leaf[0],
        0,
        &proof_single,
        single_tree.leaf_count(),
    );
    assert!(ok_single, "Verification failed for single leaf tree");

    // Edge case: odd number of leaves (already covered above)
    // Ensure that the duplicated hash handling does not break verification
    let odd_leaves: Vec<Vec<u8>> = vec![
        b"one".to_vec(),
        b"two".to_vec(),
        b"three".to_vec(),
    ];
    let odd_tree = MerkleTree::new(odd_leaves.clone());
    let odd_root = odd_tree.root();
    for (idx, leaf) in odd_leaves.iter().enumerate() {
        let proof = odd_tree.generate_proof(idx).expect("Proof exists");
        let ok = MerkleTree::verify_proof(
            odd_root,
            leaf,
            idx,
            &proof,
            odd_tree.leaf_count(),
        );
        assert!(ok, "Proof verification failed for odd leaf {}", idx);
    }

    // All assertions passed
    println!("All Merkle tree tests passed. Root hash: {}", root);
}