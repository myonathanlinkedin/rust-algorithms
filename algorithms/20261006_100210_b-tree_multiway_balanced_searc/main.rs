mod types;
mod engine;

use types::{BTreeNode, Entry};
use engine::*;

fn main() {
    println!("=== B-Tree Multiway Balanced Search Tree Node Splitter ===\n");

    // Test 1: Basic split of a leaf node
    println!("Test 1: Basic leaf node split");
    let leaf = BTreeNode {
        entries: vec![
            Entry { key: 10, value: 100 },
            Entry { key: 20, value: 200 },
            Entry { key: 30, value: 300 },
            Entry { key: 40, value: 400 },
            Entry { key: 50, value: 500 },
        ],
        children: vec![],
    };
    let result = split_node(&leaf);
    assert_eq!(result.promoted.key, 30, "Promoted key should be 30");
    assert_eq!(result.promoted.value, 300, "Promoted value should be 300");
    assert_eq!(result.left.entries.len(), 2, "Left should have 2 entries");
    assert_eq!(result.right.entries.len(), 2, "Right should have 2 entries");
    assert_eq!(result.left.entries[0].key, 10);
    assert_eq!(result.left.entries[1].key, 20);
    assert_eq!(result.right.entries[0].key, 40);
    assert_eq!(result.right.entries[1].key, 50);
    assert!(result.left.is_leaf(), "Left should be a leaf");
    assert!(result.right.is_leaf(), "Right should be a leaf");
    println!("  PASSED: Leaf split produces correct left, promoted, right\n");

    // Test 2: Split of an internal node
    println!("Test 2: Internal node split");
    let child1 = BTreeNode::new_leaf();
    let child2 = BTreeNode::new_leaf();
    let child3 = BTreeNode::new_leaf();
    let child4 = BTreeNode::new_leaf();
    let child5 = BTreeNode::new_leaf();
    let internal = BTreeNode {
        entries: vec![
            Entry { key: 10, value: 1 },
            Entry { key: 20, value: 2 },
            Entry { key: 30, value: 3 },
            Entry { key: 40, value: 4 },
            Entry { key: 50, value: 5 },
        ],
        children: vec![child1, child2, child3, child4, child5],
    };
    let result = split_node(&internal);
    assert_eq!(result.promoted.key, 30);
    assert_eq!(result.left.entries.len(), 2);
    assert_eq!(result.right.entries.len(), 2);
    assert_eq!(result.left.children.len(), 3, "Left internal should have 3 children");
    assert_eq!(result.right.children.len(), 2, "Right internal should have 2 children");
    assert!(!result.left.is_leaf(), "Left should be internal");
    assert!(!result.right.is_leaf(), "Right should be internal");
    println!("  PASSED: Internal split distributes children correctly\n");

    // Test 3: Split with odd number of entries (3 entries)
    println!("Test 3: Split with 3 entries (odd)");
    let odd_leaf = BTreeNode {
        entries: vec![
            Entry { key: 10, value: 100 },
            Entry { key: 20, value: 200 },
            Entry { key: 30, value: 300 },
        ],
        children: vec![],
    };
    let result = split_node(&odd_leaf);
    assert_eq!(result.promoted.key, 20, "Middle of 3 should be index 1");
    assert_eq!(result.left.entries.len(), 1);
    assert_eq!(result.right.entries.len(), 1);
    assert_eq!(result.left.entries[0].key, 10);
    assert_eq!(result.right.entries[0].key, 30);
    println!("  PASSED: Odd entry split works correctly\n");

    // Test 4: Split with 4 entries (even)
    println!("Test 4: Split with 4 entries (even)");
    let even_leaf = BTreeNode {
        entries: vec![
            Entry { key: 10, value: 100 },
            Entry { key: 20, value: 200 },
            Entry { key: 30, value: 300 },
            Entry { key: 40, value: 400 },
        ],
        children: vec![],
    };
    let result = split_node(&even_leaf);
    assert_eq!(result.promoted.key, 20, "Middle of 4 should be index 2");
    assert_eq!(result.left.entries.len(), 2);
    assert_eq!(result.right.entries.len(), 1);
    assert_eq!(result.left.entries[0].key, 10);
    assert_eq!(result.left.entries[1].key, 20);
    assert_eq!(result.right.entries[0].key, 30);
    println!("  PASSED: Even entry split works correctly\n");

    // Test 5: B-Tree insertion and search
    println!("Test 5: B-Tree insertion and search");
    let mut root = BTreeNode::new_leaf();
    for i in 1..=10 {
        let entry = Entry { key: i, value: i * 10 };
        root = btree_insert(&root, &entry);
    }
}
