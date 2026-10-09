use crate::types::{BTreeNode, Entry, SplitResult};

/// The minimum number of entries a node must have (excluding root).
/// For a B-Tree of order m, min_entries = ceil(m/2) - 1.
/// We use a fixed order of 5 (max 4 entries, min 2 entries for non-root).
pub const MIN_ENTRIES: usize = 2;
pub const MAX_ENTRIES: usize = 4;

/// Splits a B-Tree node that has exceeded its maximum capacity.
///
/// # Arguments
/// * `node` - The node to split (must have more than MAX_ENTRIES entries).
///
/// # Returns
/// A `SplitResult` containing the left node, the promoted entry, and the right node.
///
/// # Panics
/// Panics if the node has fewer than 3 entries (cannot be split meaningfully).
pub fn split_node(node: &BTreeNode) -> SplitResult {
    let n = node.entries.len();
    assert!(n > MAX_ENTRIES, "Node must be overfull to split, got {} entries", n);
    assert!(n >= 3, "Cannot split a node with fewer than 3 entries");

    // The middle index determines the promoted entry.
    // For even n: middle = n / 2
    // For odd n: middle = n / 2 (integer division)
    let mid = n / 2;

    // The promoted entry is the middle one.
    let promoted = node.entries[mid].clone();

    // Left node gets entries [0..mid)
    let left_entries: Vec<Entry> = node.entries[..mid].to_vec();

    // Right node gets entries [(mid+1)..n]
    let right_entries: Vec<Entry> = node.entries[(mid + 1)..].to_vec();

    // Handle children:
    // If the node is a leaf, both left and right are leaves.
    // If the node is internal, children are distributed:
    //   Left gets children [0..=mid]  (mid + 1 children)
    //   Right gets children [(mid+1)..]  (n - mid - 1 children)
    let left_children: Vec<BTreeNode>;
    let right_children: Vec<BTreeNode>;

    if node.is_leaf() {
        left_children = Vec::new();
        right_children = Vec::new();
    } else {
        // Left node gets children from index 0 to mid (inclusive)
        left_children = node.children[..=mid].to_vec();
        // Right node gets children from index mid+1 to end
        right_children = node.children[(mid + 1)..].to_vec();
    }

    let left = BTreeNode {
        entries: left_entries,
        children: left_children,
    };

    let right = BTreeNode {
        entries: right_entries,
        children: right_children,
    };

    SplitResult {
        left,
        promoted,
        right,
    }
}

/// Inserts an entry into a B-Tree node, assuming the node is not overfull.
/// Returns the updated node.
///
/// # Arguments
/// * `node` - The node to insert into.
/// * `entry` - The entry to insert.
///
/// # Returns
/// The updated node with the entry inserted in sorted order.
pub fn insert_into_node(node: &BTreeNode, entry: &Entry) -> BTreeNode {
    let mut new_node = node.clone();

    if new_node.is_leaf() {
        // Find the correct position in the sorted entries
        let pos = new_node
            .entries
            .iter()
            .position(|e| e.key > entry.key)
            .unwrap_or(new_node.entries.len());
        new_node.entries.insert(pos, entry.clone());
    } else {
        // Find which child to descend into
        let mut child_idx = new_node.entries.len();
        for (i, e) in new_node.entries.iter().enumerate() {
            if entry.key < e.key {
                child_idx = i;
                break;
            }
        }

        // Recursively insert into the child
        let child = new_node.children[child_idx].clone();
        let updated_child = insert_into_node(&child, entry);
        new_node.children[child_idx] = updated_child;
    }

    new_node
}

/// Performs a full B-Tree insert operation with splitting.
///
/// # Arguments
/// * `root` - The current root of the B-Tree.
/// * `entry` - The entry to insert.
///
/// # Returns
/// The new root of the B-Tree after insertion (may be a new root if the old root split).
pub fn btree_insert(root: &BTreeNode, entry: &Entry) -> BTreeNode {
    // First, try to insert into the root without splitting
    let mut new_root = insert_into_node(root, entry);

    // If the root is now overfull, split it
    if new_root.entry_count() > MAX_ENTRIES {
        let split = split_node(&new_root);
        // Create a new root with the promoted entry and the two split nodes as children
        new_root = BTreeNode {
            entries: vec![split.promoted],
            children: vec![split.left, split.right],
        };
    }

    new_root
}

/// Searches for an entry with the given key in the B-Tree.
///
/// # Arguments
/// * `node` - The node to search in.
/// * `key` - The key to search for.
///
/// # Returns
/// `Some(value)` if found, `None` otherwise.
pub fn btree_search(node: &BTreeNode, key: i32) -> Option<i32> {
    // Find the position where the key would be inserted
    let pos = node
        .entries
        .iter()
        .position(|e| e.key >= key)
        .unwrap_or(node.entries.len());

    // Check if the key is at position pos
    if pos < node.entries.len() && node.entries[pos].key == key {
        return Some(node.entries[pos].value);
    }

    // If this is a leaf, the key is not present
    if node.is_leaf() {
        return None;
    }

    // Recurse into the appropriate child
    let child = &node.children[pos];
    btree_search(child, key)
}

/// Validates the structural invariants of a B-Tree node.
///
/// # Arguments
/// * `node` - The node to validate.
/// * `is_root` - Whether this node is the root.
///
/// # Returns
/// `true` if all invariants hold, `false` otherwise.
pub fn validate_btree(node: &BTreeNode, is_root: bool) -> bool {
    // Check entry count constraints
    let n = node.entry_count();
    if is_root {
        // Root can have 0 to MAX_ENTRIES entries
        if n > MAX_ENTRIES {
            return false;
        }
    } else {
        // Non-root nodes must have MIN_ENTRIES to MAX_ENTRIES entries
        if n < MIN_ENTRIES || n > MAX_ENTRIES {
            return false;
        }
    }

    // Check that entries are in sorted order
    for i in 1..n {
        if node.entries[i].key <= node.entries[i - 1].key {
            return false;
        }
    }

    // Check child count consistency
    if !node.is_leaf() {
        if node.children.len() != n + 1 {
            return false;
        }
        // Recursively validate children
        for child in &node.children {
            if !validate_btree(child, false) {
                return false;
            }
        }
    } else {
        // Leaf nodes must have no children
        if !node.children.is_empty() {
            return false;
        }
    }

    true
}

/// Returns the height of the B-Tree.
///
/// # Arguments
/// * `node` - The node to measure.
///
/// # Returns
/// The height of the tree (0 for a leaf, 1 for a single internal node, etc.).
pub fn btree_height(node: &BTreeNode) -> usize {
    if node.is_leaf() {
        0
    } else {
        1 + node.children.iter().map(btree_height).max().unwrap_or(0)
    }
}

/// Collects all keys in the B-Tree in sorted order (in-order traversal).
///
/// # Arguments
/// * `node` - The node to traverse.
///
/// # Returns
/// A vector of keys in sorted order.
pub fn collect_keys_sorted(node: &BTreeNode) -> Vec<i32> {
    let mut keys = Vec::new();
    for (i, entry) in node.entries.iter().enumerate() {
        if i < node.children.len() {
            keys.extend(collect_keys_sorted(&node.children[i]));
        }
        keys.push(entry.key);
    }
    // Traverse the last child if it exists
    if !node.is_leaf() {
        keys.extend(collect_keys_sorted(&node.children[node.entries.len()]));
    }
    keys
}
