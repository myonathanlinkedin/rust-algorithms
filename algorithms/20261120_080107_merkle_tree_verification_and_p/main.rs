mod core;
use core::{MerkleTree, Hash};

fn main() {
    // Simple sanity check executed at runtime.
    let data = vec![
        b"block0".to_vec(),
        b"block1".to_vec(),
        b"block2".to_vec(),
        b"block3".to_vec(),
    ];
    let tree = MerkleTree::new(&data);
    let root = tree.root();
    // Verify each leaf's proof.
    for (i, block) in data.iter().enumerate() {
        let proof = tree.gen_proof(i);
        assert!(MerkleTree::verify_proof(block, &proof, root));
    }
    // An intentionally wrong proof should fail.
    let mut bad_proof = tree.gen_proof(0);
    // Flip the first sibling hash.
    if let Some(first) = bad_proof.first_mut() {
        first.0 = first.0.wrapping_add(1);
    }
    assert!(!MerkleTree::verify_proof(&data[0], &bad_proof, root));
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use core::MerkleTree;

    #[test]
    fn test_empty_tree() {
        let tree = MerkleTree::new(&[]);
        assert_eq!(tree.root(), 0);
        assert_eq!(tree.leaf_count(), 0);
    }

    #[test]
    fn test_single_leaf() {
        let data = vec![b"only".to_vec()];
        let tree = MerkleTree::new(&data);
        let root = tree.root();
        let proof = tree.gen_proof(0);
        // Proof for a single leaf should be empty.
        assert!(proof.is_empty());
        assert!(MerkleTree::verify_proof(&data[0], &proof, root));
    }

    #[test]
    fn test_power_of_two_leaves() {
        let data = vec![
            b"a".to_vec(),
            b"b".to_vec(),
            b"c".to_vec(),
            b"d".to_vec(),
        ];
        let tree = MerkleTree::new(&data);
        let root = tree.root();
        for (i, block) in data.iter().enumerate() {
            let proof = tree.gen_proof(i);
            assert!(MerkleTree::verify_proof(block, &proof, root));
        }
    }

    #[test]
    fn test_non_power_of_two_padding() {
        let data = vec![
            b"x".to_vec(),
            b"y".to_vec(),
            b"z".to_vec(),
        ]; // 3 leaves -> padded to 4
        let tree = MerkleTree::new(&data);
        assert_eq!(tree.leaf_count(), 4);
        let root = tree.root();
        for i in 0..3 {
            let proof = tree.gen_proof(i);
            assert!(MerkleTree::verify_proof(&data[i], &proof, root));
        }
        // The padded leaf (duplicate of last) should also verify.
        let proof = tree.gen_proof(3);
        assert!(MerkleTree::verify_proof(&data[2], &proof, root));
    }

    #[test]
    fn test_invalid_proof_detection() {
        let data = vec![
            b"alpha".to_vec(),
            b"beta".to_vec(),
            b"gamma".to_vec(),
            b"delta".to_vec(),
        ];
        let tree = MerkleTree::new(&data);
        let root = tree.root();
        let mut proof = tree.gen_proof(2);
        // Corrupt the proof by altering a sibling hash.
        if let Some((ref mut hash, _)) = proof.get_mut(0) {
            *hash = hash.wrapping_add(42);
        }
        assert!(!MerkleTree::verify_proof(&data[2], &proof, root));
    }

    #[test]
    fn test_consistency_across_instances() {
        let data = (0..8).map(|i| i.to_be_bytes().to_vec()).collect::<Vec<_>>();
        let tree1 = MerkleTree::new(&data);
        let tree2 = MerkleTree::new(&data);
        assert_eq!(tree1.root(), tree2.root());
        for i in 0..data.len() {
            let p1 = tree1.gen_proof(i);
            let p2 = tree2.gen_proof(i);
            assert_eq!(p1, p2);
            assert!(MerkleTree::verify_proof(&data[i], &p1, tree1.root()));
        }
    }
}