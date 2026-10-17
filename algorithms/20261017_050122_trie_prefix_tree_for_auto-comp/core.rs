use std::collections::HashMap;

/// A node in the Trie. Each node may terminate a word and stores its frequency.
#[derive(Debug, Clone, PartialEq)]
pub struct TrieNode {
    pub children: HashMap<char, Box<TrieNode>>,
    pub frequency: Option<u32>,

}

impl TrieNode {
    /// Creates an empty TrieNode.
    pub fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            frequency: None,
        }
    }
}

/// Trie data structure supporting insertion with frequency and prefix auto‑completion.
#[derive(Debug, Clone, PartialEq)]
pub struct Trie {
    pub root: TrieNode,

}

impl Trie {
    /// Constructs a new empty Trie.
    pub fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    /// Inserts `word` with the given `frequency`.
    ///
    /// If the word already exists, its stored frequency becomes the maximum of the
    /// existing and the new value.
    pub fn insert(&mut self, word: &str, frequency: u32) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            node = node
                .children
                .entry(ch)
                .or_insert_with(|| Box::new(TrieNode::new()));
        }
        node.frequency = Some(match node.frequency {
            Some(old) => old.max(frequency),
            None => frequency,
        });
    }

    /// Returns up to `limit` completions for `prefix`, ordered by descending frequency.
    /// Ties are broken by lexicographic order (ascending).
    pub fn autocomplete(&self, prefix: &str, limit: usize) -> Vec<(String, u32)> {
        // Locate the node representing the prefix.
        let mut node = &self.root;
        for ch in prefix.chars() {
            match node.children.get(&ch) {
                Some(child) => node = child,
                None => return Vec::new(),
            }
        }

        // Collect all words under this node.
        let mut results: Vec<(String, u32)> = Vec::new();
        let mut current = prefix.to_string();
        Self::collect(node, &mut current, &mut results);

        // Sort by frequency descending, then lexicographically ascending.
        results.sort_by(|a, b| {
            match b.1.cmp(&a.1) {
                std::cmp::Ordering::Equal => a.0.cmp(&b.0),
                other => other,
            }
        });

        if limit == 0 || results.len() <= limit {
            results
        } else {
            results.truncate(limit);
            results
        }
    }

    /// Helper recursive DFS that gathers all words reachable from `node`.
    fn collect(node: &TrieNode, current: &mut String, out: &mut Vec<(String, u32)>) {
        if let Some(freq) = node.frequency {
            out.push((current.clone(), freq));
        }

        // To guarantee deterministic order, iterate children sorted by key.
        let mut keys: Vec<char> = node.children.keys().cloned().collect();
        keys.sort_unstable();

        for ch in keys {
            if let Some(child) = node.children.get(&ch) {
                current.push(ch);
                Self::collect(child, current, out);
                current.pop();
            }
        }
    }
}