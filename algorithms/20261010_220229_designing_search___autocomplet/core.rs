use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct InvertedIndex {
    pub index: HashMap<String, Vec<u32>>,
    pub trie: Trie,
    pub word_freq: HashMap<String, u32>,

}

impl InvertedIndex {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
            trie: Trie::new(),
            word_freq: HashMap::new(),
        }
    }

    pub fn add_document(&mut self, doc_id: u32, text: &str) {
        for word in text.split_whitespace() {
            let word = word.to_lowercase();
            self.index.entry(word.clone()).or_default().push(doc_id);
            *self.word_freq.entry(word.clone()).or_insert(0) += 1;
            self.trie.insert(&word);
        }
    }

    pub fn search(&self, term: &str) -> Vec<u32> {
        self.index
            .get(&term.to_lowercase())
            .cloned()
            .unwrap_or_default()
    }

    pub fn autocomplete(&self, prefix: &str) -> Vec<(String, u32)> {
        let node_opt = self.trie.find_node(prefix);
        let mut results = Vec::new();
        if let Some(node) = node_opt {
            self.trie.collect_words(node, prefix.to_string(), &mut results, &self.word_freq);
        }
        results.sort_by(|a, b| b.1.cmp(&a.1));
        results
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TrieNode {
    pub children: HashMap<char, TrieNode>,
    pub is_word: bool,

}

impl TrieNode {
    fn new() -> Self {
        Self {
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
        Self {
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

    fn find_node<'a>(&'a self, prefix: &str) -> Option<&'a TrieNode> {
        let mut node = &self.root;
        for ch in prefix.chars() {
            match node.children.get(&ch) {
                Some(n) => node = n,
                None => return None,
            }
        }
        Some(node)
    }

    fn collect_words<'a>(
        &self,
        node: &'a TrieNode,
        current: String,
        results: &mut Vec<(String, u32)>,
        freq_map: &HashMap<String, u32>,
    ) {
        if node.is_word {
            if let Some(&freq) = freq_map.get(&current) {
                results.push((current.clone(), freq));
            }
        }
        for (&ch, child) in &node.children {
            let mut next = current.clone();
            next.push(ch);
            self.collect_words(child, next, results, freq_map);
        }
    }
}