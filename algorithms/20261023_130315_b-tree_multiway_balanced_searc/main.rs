mod core;
use core::{BTree, BTreeNode};

fn main() {
    // Simple sanity checks via asserts
    let mut tree = BTree::new(2);
    for i in 1..=7 {
        tree.insert(i, i * 10);
    }
    // Verify search
    for i in 1..=7 {
        let val = tree.search(&i).expect("Key must exist");
        assert_eq!(*val, i * 10);
    }
    // Verify inorder yields sorted keys
    let inorder = tree.inorder();
    let expected_keys: Vec<i32> = (1..=7).collect();
    let actual_keys: Vec<i32> = inorder.iter().map(|(k, _)| *k).collect();
    assert_eq!(actual_keys, expected_keys);
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_split_child_behavior() {
        // Build a node with max keys (2t-1) where t=3
        let t = 3usize;
        let mut parent = BTreeNode::new(false);
        let mut child = BTreeNode::new(true);
        // Fill child with keys 1..5 (2t-1 = 5)
        for i in 1..=5 {
            child.keys.push(i);
            child.values.push(i * 100);
        }
        parent.children.push(Box::new(child));
        // Perform split
        parent.split_child(0, t);
        // After split, parent should have 1 key (the median 3)
        assert_eq!(parent.keys, vec![3]);
        assert_eq!(parent.values, vec![300]);
        // Parent should have two children
        assert_eq!(parent.children.len(), 2);
        // Left child should have keys [1,2]
        assert_eq!(parent.children[0].keys, vec![1, 2]);
        // Right child should have keys [4,5]
        assert_eq!(parent.children[1].keys, vec![4, 5]);
    }

    #[test]
    fn test_insertion_and_structure() {
        let mut tree = BTree::new(2); // Minimum degree 2
        // Insert keys that force multiple splits
        let keys = vec![10, 20, 5, 6, 12, 30, 7, 17];
        for &k in &keys {
            tree.insert(k, k * 2);
        }
        // Verify all keys are searchable
        for &k in &keys {
            let v = tree.search(&k).expect("Key missing");
            assert_eq!(*v, k * 2);
        }
        // Verify inorder traversal is sorted
        let inorder = tree.inorder();
        let mut sorted_keys = keys.clone();
        sorted_keys.sort_unstable();
        let inorder_keys: Vec<_> = inorder.iter().map(|(k, _)| *k).collect();
        assert_eq!(inorder_keys, sorted_keys);
        // Root should not be a leaf after enough insertions
        assert!(!tree.root.leaf);
        // Height should be 2 (root + leaf level) for this set
        assert_eq!(tree.root.children.len(), 2);
    }

    #[test]
    fn test_search_nonexistent() {
        let mut tree = BTree::new(3);
        for i in 1..=15 {
            tree.insert(i, i);
        }
        assert!(tree.search(&0).is_none());
        assert!(tree.search(&16).is_none());
    }
}