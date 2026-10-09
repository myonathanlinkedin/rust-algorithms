use std::fmt;

/// Represents a single key-value pair stored within a B-Tree node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub key: i32,
    pub value: i32,

}

/// Represents a node in the B-Tree.
/// A node contains a vector of entries (keys and values) and a vector of child pointers.
/// For a leaf node, `children` is empty.
/// For an internal node, `children.len()` must be `entries.len() + 1`.
#[derive(Debug, Clone, PartialEq)]
pub struct BTreeNode {
    pub entries: Vec<Entry>,
    pub children: Vec<BTreeNode>,

}

impl BTreeNode {
    /// Creates a new empty leaf node.
    pub fn new_leaf() -> Self {
        BTreeNode {
            entries: Vec::new(),
            children: Vec::new(),
        }
    }

    /// Creates a new internal node with the given children.
    pub fn new_internal(children: Vec<BTreeNode>) -> Self {
        BTreeNode {
            entries: Vec::new(),
            children,
        }
    }

    /// Returns true if this node is a leaf (has no children).
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    /// Returns the number of entries in this node.
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }
}

impl fmt::Display for BTreeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_leaf() {
            write!(f, "Leaf[")?;
            for (i, entry) in self.entries.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}:{}", entry.key, entry.value)?;
            }
            write!(f, "]")
        } else {
            write!(f, "Internal[")?;
            for (i, entry) in self.entries.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}:{}", entry.key, entry.value)?;
            }
            write!(f, "]")?;
            write!(f, " Children: [")?;
            for (i, child) in self.children.iter().enumerate() {
                if i > 0 {
                    write!(f, " | ")?;
                }
                write!(f, "{}", child)?;
            }
            write!(f, "]")
        }
    }
}

/// Result of splitting a B-Tree node.
/// Contains the left node, the promoted entry, and the right node.
#[derive(Debug, Clone, PartialEq)]
pub struct SplitResult {
    pub left: BTreeNode,
    pub promoted: Entry,
    pub right: BTreeNode,

}
