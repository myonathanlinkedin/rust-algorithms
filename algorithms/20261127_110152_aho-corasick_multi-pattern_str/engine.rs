use super::types::{AhoCorasick, Node};
use std::collections::VecDeque;

impl AhoCorasick {
    pub fn new(patterns: Vec<String>) -> Self {
        let mut ac = AhoCorasick {
            patterns,
            nodes: Vec::new(),
        };
        ac.build();
        ac
    }

    fn build(&mut self) {
        // Root node
        self.nodes.push(Node {
            children: [None; 26],
            fail: 0,
            output: Vec::new(),
        });

        // Insert patterns into trie
        for (idx, pat) in self.patterns.iter().enumerate() {
            if pat.is_empty() {
                continue; // ignore empty patterns
            }
            let mut node = 0;
            for ch in pat.bytes() {
                let c = (ch - b'a') as usize;
                let next = match self.nodes[node].children[c] {
                    Some(n) => n,
                    None => {
                        let new_node = Node {
                            children: [None; 26],
                            fail: 0,
                            output: Vec::new(),
                        };
                        self.nodes.push(new_node);
                        let new_index = self.nodes.len() - 1;
                        self.nodes[node].children[c] = Some(new_index);
                        new_index
                    }
                };
                node = next;
            }
            self.nodes[node].output.push(idx);
        }

        // Build failure links using BFS
        let mut queue: VecDeque<usize> = VecDeque::new();

        // Depth 1 nodes
        for c in 0..26 {
            if let Some(child) = self.nodes[0].children[c] {
                self.nodes[child].fail = 0;
                queue.push_back(child);
            }
        }

        while let Some(r) = queue.pop_front() {
            for c in 0..26 {
                if let Some(u) = self.nodes[r].children[c] {
                    let mut v = self.nodes[r].fail;
                    while v != 0 && self.nodes[v].children[c].is_none() {
                        v = self.nodes[v].fail;
                    }
                    if let Some(f) = self.nodes[v].children[c] {
                        self.nodes[u].fail = f;
                    } else {
                        self.nodes[u].fail = 0;
                    }
                    // Merge output from fail state
                    let fail_output = self.nodes[self.nodes[u].fail].output.clone();
                    self.nodes[u].output.extend(fail_output);
                    queue.push_back(u);
                }
            }
        }
    }

    pub fn search(&self, text: &str) -> Vec<(usize, usize)> {
        let mut results = Vec::new();
        let mut state = 0;
        for (i, ch) in text.bytes().enumerate() {
            if ch < b'a' || ch > b'z' {
                state = 0;
                continue;
            }
            let c = (ch - b'a') as usize;
            while state != 0 && self.nodes[state].children[c].is_none() {
                state = self.nodes[state].fail;
            }
            if let Some(next) = self.nodes[state].children[c] {
                state = next;
            } else {
                state = 0;
            }
            for &pat_idx in &self.nodes[state].output {
                let pat_len = self.patterns[pat_idx].len();
                let pos = i + 1 - pat_len;
                results.push((pat_idx, pos));
            }
        }
        results
    }
}