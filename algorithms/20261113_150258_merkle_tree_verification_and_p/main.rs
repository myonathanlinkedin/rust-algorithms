mod core;
use core::{MerkleProof, MerkleTree, verify_proof};

fn main() {
    // Simple demonstration with four leaves.
    let leaves = vec![
        b"apple".to_vec(),
        b"banana".to_vec(),
        b"cherry".to_vec(),
        b"date".to_vec(),
    ];

    let tree = MerkleTree::new(leaves.clone());
    let root = tree.root().to_vec();

    // Verify each leaf against its proof.
    for (i, leaf) in leaves.iter().enumerate() {
        let proof = tree.generate_proof(i);
        assert!(verify_proof(&root, leaf, i, &proof));
    }

    // Negative test: tampered leaf should fail verification.
    let bad_leaf = b"tampered".to_vec();
    let proof = tree.generate_proof(0);
    assert!(!verify_proof(&root, &bad_leaf, 0, &proof));

    println!("All Merkle tree assertions passed.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even_number_of_leaves() {
        let data = vec![
            b"one".to_vec(),
            b"two".to_vec(),
            b"three".to_vec(),
            b"four".to_vec(),
        ];
        let tree = MerkleTree::new(data.clone());
        let root = tree.root().to_vec();

        for (i, leaf) in data.iter().enumerate() {
            let proof = tree.generate_proof(i);
            assert!(verify_proof(&root, leaf, i, &proof));
        }
    }

    #[test]
    fn test_odd_number_of_leaves() {
        // An odd count forces the duplication rule at the lowest level.
        let data = vec![
            b"alpha".to_vec(),
            b"beta".to_vec(),
            b"gamma".to_vec(),
        ];
        let tree = MerkleTree::new(data.clone());
        let root = tree.root().to_vec();

        for (i, leaf) in data.iter().enumerate() {
            let proof = tree.generate_proof(i);
            assert!(verify_proof(&root, leaf, i, &proof));
        }
    }

    #[test]
    fn test_invalid_proof_detection() {
        let data = vec![
            b"red".to_vec(),
            b"green".to_vec(),
            b"blue".to_vec(),
            b"yellow".to_vec(),
        ];
        let tree = MerkleTree::new(data.clone());
        let root = tree.root().to_vec();

        // Generate a valid proof for the first leaf.
        let mut proof = tree.generate_proof(0);
        // Corrupt the first sibling hash.
        proof.sibling_hashes[0][0] ^= 0xFF;

        assert!(!verify_proof(&root, &data[0], 0, &proof));
    }

    #[test]
    fn test_proof_mismatch_index() {
        let data = vec![
            b"cat".to_vec(),
            b"dog".to_vec(),
        ];
        let tree = MerkleTree::new(data.clone());
        let root = tree.root().to_vec();

        let proof = tree.generate_proof(0);
        // Attempt to verify leaf 0 with index 1 – should fail.
        assert!(!verify_proof(&root, &data[0], 1, &proof));
    }
}