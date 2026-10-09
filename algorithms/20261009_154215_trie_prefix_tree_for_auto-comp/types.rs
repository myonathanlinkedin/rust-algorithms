pub use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct TrieNode {
    pub children: HashMap<char, Box<TrieNode>>,
    pub frequency: usize,
    pub is_word: bool,

}

impl TrieNode {
    pub fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            frequency: 0,
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
}