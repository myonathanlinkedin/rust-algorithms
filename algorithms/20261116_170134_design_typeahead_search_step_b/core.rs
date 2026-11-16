use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct TrieNode {
    pub children: HashMap<char, Box<TrieNode>>,
    pub top_suggestions: Vec<(String, usize)>, // (word, frequency)
    pub is_word: bool,

}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            top_suggestions: Vec::new(),
            is_word: false,
        }
    }

    fn update_top(&mut self, word: &str, freq: usize) {
        self.top_suggestions.push((word.to_string(), freq));
        // Sort by descending frequency, then ascending lexical order for determinism
        self.top_suggestions
            .sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        // Remove duplicate words, keeping the first (highest freq) occurrence
        let mut seen: HashSet<String> = HashSet::new();
        self.top_suggestions.retain(|(w, _)| seen.insert(w.clone()));
        // Keep only the top 10 entries
        if self.top_suggestions.len() > 10 {
            self.top_suggestions.truncate(10);
        }
    }
}

#[derive(Debug)]
pub struct Trie {
    pub root: TrieNode,

}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    /// Insert a word with its associated frequency.
    /// If the word already exists, its frequency is overwritten.
    pub fn insert(&mut self, word: &str, freq: usize) {
        let mut node = &mut self.root;
        node.update_top(word, freq);
        for ch in word.chars() {
            node = node
                .children
                .entry(ch)
                .or_insert_with(|| Box::new(TrieNode::new()));
            node.update_top(word, freq);
        }
        node.is_word = true;
    }

    /// Return up to 10 suggestions for the given prefix,
    /// ordered by descending frequency then lexical order.
    pub fn suggestions(&self, prefix: &str) -> Vec<String> {
        let mut node = &self.root;
        for ch in prefix.chars() {
            match node.children.get(&ch) {
                Some(child) => node = child,
                None => return Vec::new(),
            }
        }
        node.top_suggestions
            .iter()
            .map(|(w, _)| w.clone())
            .collect()
    }
}