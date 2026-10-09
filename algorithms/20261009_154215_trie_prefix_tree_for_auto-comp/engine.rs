use crate::types::{Trie, TrieNode};
use std::collections::HashMap;

impl Trie {
    /// Insert a word into the trie, incrementing its frequency by 1.
    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            node = node
                .children
                .entry(ch)
                .or_insert_with(|| Box::new(TrieNode::new()));
        }
        node.is_word = true;
        node.frequency += 1;
    }

    /// Return up to `limit` completions for `prefix`, ordered by descending frequency
    /// and then lexicographically.
    pub fn autocomplete(&self, prefix: &str, limit: usize) -> Vec<(String, usize)> {
        // Locate the node representing the prefix.
        let mut node = &self.root;
        for ch in prefix.chars() {
            match node.children.get(&ch) {
                Some(child) => node = child,
                None => return Vec::new(),
            }
        }

        // Collect all words under this node.
        let mut results: Vec<(String, usize)> = Vec::new();
        let mut current = String::from(prefix);
        Self::collect(node, &mut current, &mut results);

        // Sort by frequency descending, then lexicographically.
        results.sort_by(|a, b| {
            b.1.cmp(&a.1) // descending frequency
                .then_with(|| a.0.cmp(&b.0)) // ascending lexical
        });

        results.truncate(limit);
        results
    }

    fn collect(node: &TrieNode, current: &mut String, out: &mut Vec<(String, usize)>) {
        if node.is_word {
            out.push((current.clone(), node.frequency));
        }
        for (ch, child) in &node.children {
            current.push(*ch);
            Self::collect(child, current, out);
            current.pop();
        }
    }
}