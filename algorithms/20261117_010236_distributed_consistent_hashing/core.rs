use std::collections::{BTreeMap, HashMap};
use std::hash::{Hash, Hasher};
use std::fmt::Debug;
use std::collections::hash_map::DefaultHasher;

/// ConsistentHashRouter implements a deterministic consistent hashing ring with virtual nodes.
///
/// * `T` – type of the real node identifier (must be hashable, comparable and clonable).
pub struct ConsistentHashRouter<T>
where
    T: Clone + Debug + PartialEq + Eq + Hash,
{
    /// Number of virtual nodes (replicas) per real node.
    replicas: usize,
    /// Sorted map from hash value to real node.
    ring: BTreeMap<u64, T>,
    /// Reverse lookup from real node to its virtual node hashes (for removal).
    node_hashes: HashMap<T, Vec<u64>>,
}

impl<T> ConsistentHashRouter<T>
where
    T: Clone + Debug + PartialEq + Eq + Hash,
{
    /// Creates a new router with the given number of virtual node replicas.
    ///
    /// # Panics
    ///
    /// Panics if `replicas` is zero.
    pub fn new(replicas: usize) -> Self {
        assert!(replicas > 0, "replicas must be greater than zero");
        ConsistentHashRouter {
            replicas,
            ring: BTreeMap::new(),
            node_hashes: HashMap::new(),
        }
    }

    /// Adds a real node to the ring, creating `replicas` virtual nodes.
    ///
    /// If the node already exists, this call is a no‑op.
    pub fn add_node(&mut self, node: T) {
        if self.node_hashes.contains_key(&node) {
            return;
        }
        let mut hashes = Vec::with_capacity(self.replicas);
        for i in 0..self.replicas {
            // Combine node identifier with replica index to obtain distinct hashes.
            let virtual_id = format!("{:?}-{}", node, i);
            let hash = Self::hash(&virtual_id);
            // In the unlikely event of a hash collision, we simply overwrite;
            // the algorithm's correctness is unaffected because the key will map
            // to the same node.
            self.ring.insert(hash, node.clone());
            hashes.push(hash);
        }
        self.node_hashes.insert(node, hashes);
    }

    /// Removes a real node and all its virtual nodes from the ring.
    ///
    /// If the node does not exist, this call is a no‑op.
    pub fn remove_node(&mut self, node: &T) {
        if let Some(hashes) = self.node_hashes.remove(node) {
            for h in hashes {
                self.ring.remove(&h);
            }
        }
    }

    /// Returns the real node responsible for the given key.
    ///
    /// Returns `None` if the ring is empty.
    pub fn get_node<K>(&self, key: &K) -> Option<T>
    where
        K: Hash,
    {
        if self.ring.is_empty() {
            return None;
        }
        let key_hash = Self::hash(key);
        // Find the first virtual node with hash >= key_hash.
        // If none exists, wrap around to the first entry.
        let entry = self.ring.range(key_hash..).next().or_else(|| self.ring.iter().next());
        entry.map(|(_, node)| node.clone())
    }

    /// Internal deterministic hash function using `DefaultHasher`.
    fn hash<U: Hash>(item: &U) -> u64 {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        hasher.finish()
    }

    /// Returns the number of real nodes currently in the ring.
    pub fn node_count(&self) -> usize {
        self.node_hashes.len()
    }

    /// Returns the total number of virtual nodes (size of the ring).
    pub fn virtual_node_count(&self) -> usize {
        self.ring.len()
    }
}