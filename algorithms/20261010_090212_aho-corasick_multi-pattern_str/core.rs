use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub struct AhoCorasick {
    pub nodes: Vec<Node>,
    pub patterns: Vec<Vec<u8>>,

}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub next: [Option<usize>; 256],
    pub fail: usize,
    pub output: Vec<usize>,

}

impl Node {
    fn new() -> Self {
        Node {
            next: [None; 256],
            fail: 0,
            output: Vec::new(),
        }
    }
}

impl AhoCorasick {
    /// Build automaton from a slice of patterns.
    pub fn new(patterns: &[&str]) -> Self {
        let mut nodes = Vec::new();
        nodes.push(Node::new()); // root

        let mut pat_bytes = Vec::new();
        for pat in patterns {
            let bytes = pat.as_bytes().to_vec();
            pat_bytes.push(bytes.clone());
            let mut state = 0;
            for &b in &bytes {
                let next_state = match nodes[state].next[b as usize] {
                    Some(idx) => idx,
                    None => {
                        let idx = nodes.len();
                        nodes.push(Node::new());
                        nodes[state].next[b as usize] = Some(idx);
                        idx
                    }
                };
                state = next_state;
            }
            nodes[state].output.push(pat_bytes.len() - 1);
        }

        // Build failure links
        let mut queue = VecDeque::new();
        // Initialize depth 1 nodes
        for b in 0..256usize {
            if let Some(child) = nodes[0].next[b] {
                nodes[child].fail = 0;
                queue.push_back(child);
            }
        }

        while let Some(r) = queue.pop_front() {
            for b in 0..256usize {
                if let Some(s) = nodes[r].next[b] {
                    let mut f = nodes[r].fail;
                    while f != 0 && nodes[f].next[b].is_none() {
                        f = nodes[f].fail;
                    }
                    if let Some(f_next) = nodes[f].next[b] {
                        nodes[s].fail = f_next;
                    } else {
                        nodes[s].fail = 0;
                    }
                    let fail_output = nodes[nodes[s].fail].output.clone();
                    nodes[s].output.extend(fail_output);
                    queue.push_back(s);
                }
            }
        }

        AhoCorasick {
            nodes,
            patterns: pat_bytes,
        }
    }

    /// Search `text` and return vector of (position, pattern_index).
    /// Position is the start index of the match in `text`.
    pub fn search(&self, text: &str) -> Vec<(usize, usize)> {
        let mut results = Vec::new();
        let bytes = text.as_bytes();
        let mut state = 0;
        for (i, &b) in bytes.iter().enumerate() {
            while state != 0 && self.nodes[state].next[b as usize].is_none() {
                state = self.nodes[state].fail;
            }
            if let Some(next_state) = self.nodes[state].next[b as usize] {
                state = next_state;
            } else {
                state = 0;
            }
            for &pat_idx in &self.nodes[state].output {
                let pat_len = self.patterns[pat_idx].len();
                if i + 1 >= pat_len {
                    results.push((i + 1 - pat_len, pat_idx));
                }
            }
        }
        results
    }
}