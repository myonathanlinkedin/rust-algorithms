use crate::types::Node;

/// Split the treap rooted at `root` into two treaps:
/// left contains keys < `key`, right contains keys >= `key`.
pub fn split(
    root: Option<Box<Node>>,
    key: i32,
) -> (Option<Box<Node>>, Option<Box<Node>>) {
    match root {
        None => (None, None),
        Some(mut node) => {
            if node.key < key {
                let (l, r) = split(node.right.take(), key);
                node.right = l;
                (Some(node), r)
            } else {
                let (l, r) = split(node.left.take(), key);
                node.left = r;
                (l, Some(node))
            }
        }
    }
}

/// Merge two treaps `left` and `right` where every key in `left` < every key in `right`.
pub fn merge(
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
) -> Option<Box<Node>> {
    match (left, right) {
        (None, r) => r,
        (l, None) => l,
        (Some(mut l_node), Some(mut r_node)) => {
            if l_node.priority < r_node.priority {
                let merged = merge(l_node.right.take(), Some(r_node));
                l_node.right = merged;
                Some(l_node)
            } else {
                let merged = merge(Some(l_node), r_node.left.take());
                r_node.left = merged;
                Some(r_node)
            }
        }
    }
}

/// Insert `node` into the treap `root`. Assumes `node.key` does not already exist.
pub fn insert_node(
    root: Option<Box<Node>>,
    node: Box<Node>,
) -> Option<Box<Node>> {
    let (left, right) = split(root, node.key);
    let merged_left = merge(left, Some(node));
    merge(merged_left, right)
}

/// Erase `key` from the treap `root`. If the key does not exist, returns the original treap.
pub fn erase_node(root: Option<Box<Node>>, key: i32) -> Option<Box<Node>> {
    // Split into <key and >=key
    let (left, middle) = split(root, key);
    // Split middle into =key and >key
    let (equal, right) = split(middle, key + 1);
    // `equal` holds the node with `key` if it existed; drop it.
    merge(left, right)
}

/// Search for `key` in the treap.
pub fn contains_node(root: &Option<Box<Node>>, key: i32) -> bool {
    let mut cur = root;
    while let Some(node) = cur {
        if key == node.key {
            return true;
        } else if key < node.key {
            cur = &node.left;
        } else {
            cur = &node.right;
        }
    }
    false
}

/// In‑order traversal collecting keys into `out`.
pub fn inorder_traverse(root: &Option<Box<Node>>, out: &mut Vec<i32>) {
    if let Some(node) = root {
        inorder_traverse(&node.left, out);
        out.push(node.key);
        inorder_traverse(&node.right, out);
    }
}