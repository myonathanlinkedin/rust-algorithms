use std::collections::HashMap;

/// A state in the suffix automaton.
#[derive(Debug, Clone, PartialEq)]
pub struct State {
    /// Length of the longest string in the equivalence class.
    pub len: usize,
    /// Suffix link (None for the root).
    pub link: Option<usize>,
    /// Transitions: character -> state index.
    pub next: HashMap<char, usize>,

}

/// Suffix Automaton (SAM) for a given string.
/// Allows linear‑time construction and O(|p|) substring queries.
#[derive(Debug, Clone, PartialEq)]
pub struct SuffixAutomaton {
    /// All states; state 0 is the initial root.
    pub states: Vec<State>,
    /// Index of the state representing the whole string processed so far.
    pub last: usize,

}

impl SuffixAutomaton {
    /// Creates an empty automaton containing only the root state.
    pub fn new() -> Self {
        let root = State {
            len: 0,
            link: None,
            next: HashMap::new(),
        };
        Self {
            states: vec![root],
            last: 0,
        }
    }

    /// Extends the automaton with a new character `c`.
    /// Runs in amortized O(1) time.
    pub fn extend(&mut self, c: char) {
        let cur = self.states.len();
        let mut new_state = State {
            len: self.states[self.last].len + 1,
            link: None,
            next: HashMap::new(),
        };
        self.states.push(new_state);

        // Step 1: add transition from existing states that lack `c`.
        let mut p_opt = Some(self.last);
        while let Some(p) = p_opt {
            if self.states[p].next.contains_key(&c) {
                break;
            }
            self.states[p].next.insert(c, cur);
            p_opt = self.states[p].link;
        }

        // Step 2: set suffix link for `cur`.
        if let Some(p) = p_opt {
            let q = self.states[p].next[&c];
            if self.states[p].len + 1 == self.states[q].len {
                self.states[cur].link = Some(q);
            } else {
                // Clone state `q` into `clone`.
                let clone = self.states.len();
                let mut clone_state = State {
                    len: self.states[p].len + 1,
                    link: self.states[q].link,
                    next: self.states[q].next.clone(),
                };
                self.states.push(clone_state);

                // Redirect transitions pointing to `q` to `clone`.
                let mut pp_opt = Some(p);
                while let Some(pp) = pp_opt {
                    if self.states[pp].next.get(&c) == Some(&q) {
                        self.states[pp].next.insert(c, clone);
                        pp_opt = self.states[pp].link;
                    } else {
                        break;
                    }
                }

                self.states[q].link = Some(clone);
                self.states[cur].link = Some(clone);
            }
        } else {
            // No state had a transition with `c`; link to root.
            self.states[cur].link = Some(0);
        }

        self.last = cur;
    }

    /// Builds a suffix automaton for the entire string `s`.
    pub fn build(s: &str) -> Self {
        let mut sam = Self::new();
        for ch in s.chars() {
            sam.extend(ch);
        }
        sam
    }

    /// Returns `true` iff `pattern` occurs as a substring of the original string.
    /// Runs in O(|pattern|) time.
    pub fn contains(&self, pattern: &str) -> bool {
        let mut v = 0usize; // start at root
        for ch in pattern.chars() {
            match self.states[v].next.get(&ch) {
                Some(&next_state) => v = next_state,
                None => return false,
            }
        }
        true
    }
}