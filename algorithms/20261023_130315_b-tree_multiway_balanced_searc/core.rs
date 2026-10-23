use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
pub struct BTreeNode<K: Ord + Clone, V: Clone> {
    pub keys: Vec<K>,
    pub values: Vec<V>,
    pub children: Vec<Box<BTreeNode<K, V>>>,
    pub leaf: bool,

}

impl<K: Ord + Clone, V: Clone> BTreeNode<K, V> {
    pub fn new(leaf: bool) -> Self {
        BTreeNode {
            keys: Vec::new(),
            values: Vec::new(),
            children: Vec::new(),
            leaf,
        }
    }

    // Split the child at index i of self. t is the minimum degree.
    pub fn split_child(&mut self, i: usize, t: usize) {
        // child to split
        let mut y = self.children[i].as_mut();

        // Create new node z which will hold t-1 keys of y
        let mut z = BTreeNode::new(y.leaf);
        // Move keys[t..2t-1] to z
        z.keys = y.keys.split_off(t);
        z.values = y.values.split_off(t);
        // If not leaf, move the corresponding children
        if !y.leaf {
            let moved_children = y.children.split_off(t);
            z.children = moved_children;
        }

        // Median key to move up
        let median_key = y.keys.pop().expect("Child must have median key");
        let median_value = y.values.pop().expect("Child must have median value");

        // Insert median into self
        self.keys.insert(i, median_key);
        self.values.insert(i, median_value);
        // Insert new child z into self.children at i+1
        self.children.insert(i + 1, Box::new(z));
    }

    // Insert a key into a node that is guaranteed not full.
    pub fn insert_non_full(&mut self, key: K, value: V, t: usize) {
        let mut i = self.keys.len();
        if self.leaf {
            // Find position to insert
            while i > 0 && self.keys[i - 1] > key {
                i -= 1;
            }
            self.keys.insert(i, key);
            self.values.insert(i, value);
        } else {
            // Find child to descend
            while i > 0 && self.keys[i - 1] > key {
                i -= 1;
            }
            // Ensure child is not full
            if self.children[i].keys.len() == 2 * t - 1 {
                self.split_child(i, t);
                // After split, decide which of the two children to descend
                if self.keys[i].cmp(&key) == Ordering::Less {
                    i += 1;
                }
            }
            self.children[i].insert_non_full(key, value, t);
        }
    }

    // Search for a key, returning reference to value if found.
    pub fn search(&self, key: &K) -> Option<&V> {
        let mut i = 0;
        while i < self.keys.len() && key > &self.keys[i] {
            i += 1;
        }
        if i < self.keys.len() && &self.keys[i] == key {
            return Some(&self.values[i]);
        }
        if self.leaf {
            None
        } else {
            self.children[i].search(key)
        }
    }

    // Inorder traversal collecting (K,V) pairs.
    pub fn inorder(&self, out: &mut Vec<(K, V)>) {
        if self.leaf {
            for (k, v) in self.keys.iter().cloned().zip(self.values.iter().cloned()) {
                out.push((k, v));
            }
        } else {
            for idx in 0..self.keys.len() {
                self.children[idx].inorder(out);
                out.push((self.keys[idx].clone(), self.values[idx].clone()));
            }
            // Last child
            self.children[self.keys.len()].inorder(out);
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BTree<K: Ord + Clone, V: Clone> {
    pub root: Box<BTreeNode<K, V>>,
    pub t: usize, // Minimum degree

}

impl<K: Ord + Clone, V: Clone> BTree<K, V> {
    pub fn new(t: usize) -> Self {
        assert!(t >= 2, "Minimum degree must be at least 2");
        BTree {
            root: Box::new(BTreeNode::new(true)),
            t,
        }
    }

    pub fn search(&self, key: &K) -> Option<&V> {
        self.root.search(key)
    }

    pub fn insert(&mut self, key: K, value: V) {
        if self.root.keys.len() == 2 * self.t - 1 {
            // Root full, need to split
            let mut s = BTreeNode::new(false);
            // Old root becomes child 0 of new root
            s.children.push(std::mem::replace(&mut self.root, Box::new(BTreeNode::new(true))));
            s.split_child(0, self.t);
            self.root = Box::new(s);
        }
        self.root.insert_non_full(key, value, self.t);
    }

    pub fn inorder(&self) -> Vec<(K, V)> {
        let mut out = Vec::new();
        self.root.inorder(&mut out);
        out
    }
}