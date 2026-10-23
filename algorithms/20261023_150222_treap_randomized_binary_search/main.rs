mod types;
mod engine;

use types::Treap;

fn main() {
    // Basic insertion and ordering test
    let mut treap = Treap::new();
    let keys = vec![5, 3, 8, 1, 4, 7, 9];
    for &k in &keys {
        treap.insert(k);
    }
    let inorder = treap.inorder();
    assert_eq!(inorder, vec![1, 3, 4, 5, 7, 8, 9]);

    // Duplicate insertion should be ignored
    treap.insert(5);
    assert_eq!(treap.inorder(), inorder);

    // Contains checks
    for &k in &keys {
        assert!(treap.contains(k));
    }
    assert!(!treap.contains(0));
    assert!(!treap.contains(10));

    // Deletion of leaf, internal node, and root
    treap.erase(1); // leaf
    assert!(!treap.contains(1));
    treap.erase(5); // root (after previous deletions)
    assert!(!treap.contains(5));
    treap.erase(8); // internal node with two children
    assert!(!treap.contains(8));

    // Verify remaining ordering
    let remaining = treap.inorder();
    assert_eq!(remaining, vec![3, 4, 7, 9]);

    // Stress test: insert many sequential keys and then delete them
    let mut big = Treap::new();
    for i in 0..1000 {
        big.insert(i);
    }
    for i in 0..1000 {
        assert!(big.contains(i));
    }
    for i in (0..1000).step_by(2) {
        big.erase(i);
    }
    for i in 0..1000 {
        if i % 2 == 0 {
            assert!(!big.contains(i));
        } else {
            assert!(big.contains(i));
        }
    }
    // Final inorder should be all odd numbers in order
    let final_inorder = big.inorder();
    let expected: Vec<i32> = (0..1000).filter(|x| x % 2 == 1).collect();
    assert_eq!(final_inorder, expected);
}