use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
pub struct TrieNode {
    pub children: HashMap<char, TrieNode>,
    pub is_word: bool,

}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_word: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Trie {
    pub root: TrieNode,

}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            node = node.children.entry(ch).or_insert_with(TrieNode::new);
        }
        node.is_word = true;
    }

    pub fn contains(&self, word: &str) -> bool {
        let mut node = &self.root;
        for ch in word.chars() {
            match node.children.get(&ch) {
                Some(next) => node = next,
                None => return false,
            }
        }
        node.is_word
    }

    fn generate_edits1(&self, word: &str) -> Vec<String> {
        let letters: Vec<char> = ('a'..='z').collect();
        let chars: Vec<char> = word.chars().collect();
        let mut edits = Vec::new();

        // Deletions
        for i in 0..chars.len() {
            let mut s = String::new();
            for (j, &c) in chars.iter().enumerate() {
                if j != i {
                    s.push(c);
                }
            }
            edits.push(s);
        }

        // Transpositions
        for i in 0..chars.len().saturating_sub(1) {
            let mut s = String::new();
            for (j, &c) in chars.iter().enumerate() {
                if j == i {
                    s.push(chars[i + 1]);
                } else if j == i + 1 {
                    s.push(c);
                } else {
                    s.push(c);
                }
            }
            edits.push(s);
        }

        // Replacements
        for i in 0..chars.len() {
            for &c in &letters {
                if c != chars[i] {
                    let mut s = String::new();
                    for (j, &ch) in chars.iter().enumerate() {
                        if j == i {
                            s.push(c);
                        } else {
                            s.push(ch);
                        }
                    }
                    edits.push(s);
                }
            }
        }

        // Insertions
        for i in 0..=chars.len() {
            for &c in &letters {
                let mut s = String::new();
                for (j, &ch) in chars.iter().enumerate() {
                    if j == i {
                        s.push(c);
                    }
                    s.push(ch);
                }
                if i == chars.len() {
                    s.push(c);
                }
                edits.push(s);
            }
        }

        edits
    }

    pub fn suggestions(&self, word: &str) -> Vec<String> {
        let mut candidates: HashSet<String> = HashSet::new();
        for edit in self.generate_edits1(word) {
            if self.contains(&edit) {
                candidates.insert(edit);
            }
        }
        let mut result: Vec<String> = candidates.into_iter().collect();
        result.sort();
        result
    }
}