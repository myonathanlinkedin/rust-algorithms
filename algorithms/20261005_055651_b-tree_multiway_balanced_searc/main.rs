use std::cmp::Ordering;

#[derive(Debug, Clone)]
struct BTreeNode<K: Ord + Clone, V: Clone> {
    keys: Vec<K>,
    values: Vec<V>,
    children: Vec<Box<BTreeNode<K, V>>>,
    leaf: bool,
}

impl<K: Ord + Clone, V: Clone> BTreeNode<K, V> {
    fn new(leaf: bool) -> Self {
        BTreeNode {
            keys: Vec::new(),
            values: Vec::new(),
            children: Vec::new(),
            leaf,
        }
    }

    fn is_full(&self, t: usize) -> bool {
        self.keys.len() == 2 * t - 1
    }

    // Split the child at index i (which must be full) into two nodes.
    // After split, self will gain the middle key/value and a new right sibling.
    fn split_child(&mut self, i: usize, t: usize) {
        // Extract the full child.
        let full_child = &mut self.children[i];
        assert!(full_child.is_full(t), "Child to split must be full");

        // New node that will hold keys > middle.
        let mut new_node = BTreeNode::new(full_child.leaf);

        // Move keys and values after the middle to new_node.
        // full_child.keys has length 2t-1.
        // Keys at positions t..(2t-1) go to new_node.
        new_node.keys = full_child.keys.split_off(t);
        new_node.values = full_child.values.split_off(t);

        // The middle key/value stays in full_child for now.
        let middle_key = full_child.keys.pop().unwrap();
        let middle_value = full_child.values.pop().unwrap();

        // If not leaf, move the corresponding children.
        if !full_child.leaf {
            // Children count is 2t, split after t children.
            new_node.children = full_child.children.split_off(t);
        }

        // Insert middle key/value into self at position i.
        self.keys.insert(i, middle_key);
        self.values.insert(i, middle_value);
        // Insert new_node as a child right after the split child.
        self.children.insert(i + 1, Box::new(new_node));
    }

    // Helper to insert a key/value into a non-full node.
    fn insert_non_full(&mut self, key: K, value: V, t: usize) {
        let mut idx = self.keys.len();
        while idx > 0 && key.cmp(&self.keys[idx - 1]) == Ordering::Less {
            idx -= 1;
        }

        if self.leaf {
            self.keys.insert(idx, key);
            self.values.insert(idx, value);
        } else {
            // Ensure the child we descend into is not full.
            if self.children[idx].is_full(t) {
                self.split_child(idx, t);
                // After split, decide which of the two children to descend.
                if key > self.keys[idx] {
                    idx += 1;
                }
            }
            self.children[idx].insert_non_full(key, value, t);
        }
    }
}

#[derive(Debug)]
struct BTree<K: Ord + Clone, V: Clone> {
    root: Option<Box<BTreeNode<K, V>>>,
    t: usize, // Minimum degree
}

impl<K: Ord + Clone, V: Clone> BTree<K, V> {
    fn new(t: usize) -> Self {
        assert!(t >= 2, "Minimum degree must be at least 2");
        BTree { root: None, t }
    }

    fn insert(&mut self, key: K, value: V) {
        if let Some(root) = &mut self.root {
            if root.is_full(self.t) {
                // Create new root and split old root.
                let mut new_root = BTreeNode::new(false);
                new_root.children.push(root.clone());
                new_root.split_child(0, self.t);
                self.root = Some(Box::new(new_root));
            }
        } else {
            // Empty tree: create a leaf root.
            self.root = Some(Box::new(BTreeNode::new(true)));
        }

        // Safe to unwrap because root now exists.
        self.root
            .as_mut()
            .unwrap()
            .insert_non_full(key, value, self.t);
    }

    // Simple search for testing purposes.
    fn get(&self, key: &K) -> Option<V> {
        fn search<K: Ord + Clone, V: Clone>(node: &BTreeNode<K, V>, key: &K) -> Option<V> {
            let mut i = 0;
            while i < node.keys.len() && key > &node.keys[i] {
                i += 1;
            }
            if i < node.keys.len() && key == &node.keys[i] {
                return Some(node.values[i].clone());
            }
            if node.leaf {
                None
            } else {
                search(&node.children[i], key)
            }
        }
        self.root.as_ref().and_then(|r| search(r, key))
    }
}

fn main() {
    // Test splitting a full leaf node.
    let t = 3; // Minimum degree => max keys = 5
    let mut parent = BTreeNode::<i32, i32>::new(false);
    let mut full_child = BTreeNode::new(true);
    for i in 1..=5 {
        full_child.keys.push(i);
        full_child.values.push(i * 10);
    }
    parent.children.push(Box::new(full_child));
    parent.split_child(0, t);
    // After split: parent should have 1 key (the middle = 3)
    assert_eq!(parent.keys, vec![3]);
    assert_eq!(parent.values, vec![30]);
    // Left child should have keys [1,2]
    let left = &parent.children[0];
    assert_eq!(left.keys, vec![1, 2]);
    assert_eq!(left.values, vec![10, 20]);
    // Right child should have keys [4,5]
    let right = &parent.children[1];
    assert_eq!(right.keys, vec![4, 5]);
    assert_eq!(right.values, vec![40, 50]);

    // Test full B-Tree insertion causing splits up to root.
    let mut tree = BTree::new(2); // t=2 => max keys per node = 3
    for i in 1..=10 {
        tree.insert(i, i * 100);
    }
    // Verify all keys are present.
    for i in 1..=10 {
        assert_eq!(tree.get(&i), Some(i * 100));
    }

    // Verify tree height increased (root should have at least 2 keys after many inserts)
    let root = tree.root.unwrap();
    assert!(root.keys.len() >= 2);
}
