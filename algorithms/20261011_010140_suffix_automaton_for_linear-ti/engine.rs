use crate::types::State;
use std::collections::HashMap;

#[derive(Debug)]
pub struct SuffixAutomaton {
    pub states: Vec<State>,
    pub last: usize,

}

impl SuffixAutomaton {
    /// Create an empty suffix automaton (only the root state).
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

    /// Extend the automaton with a new character `c`.
    pub fn extend(&mut self, c: char) {
        let cur = self.states.len();
        let new_state = State {
            len: self.states[self.last].len + 1,
            link: None,
            next: HashMap::new(),
        };
        self.states.push(new_state);

        // Step 1: add transition from existing states that lack `c`.
        let mut p_opt = Some(self.last);
        while let Some(p) = p_opt {
            if !self.states[p].next.contains_key(&c) {
                self.states[p].next.insert(c, cur);
                p_opt = self.states[p].link;
            } else {
                break;
            }
        }

        // Step 2: set suffix link for the new state.
        if let Some(p) = p_opt {
            let q = self.states[p].next[&c];
            if self.states[p].len + 1 == self.states[q].len {
                self.states[cur].link = Some(q);
            } else {
                // Clone state `q` into `clone`.
                let clone = self.states.len();
                let cloned_state = State {
                    len: self.states[p].len + 1,
                    link: self.states[q].link,
                    next: self.states[q].next.clone(),
                };
                self.states.push(cloned_state);

                // Redirect links.
                self.states[q].link = Some(clone);
                self.states[cur].link = Some(clone);

                // Fix transitions that pointed to `q`.
                let mut pp_opt = Some(p);
                while let Some(pp) = pp_opt {
                    if let Some(&dest) = self.states[pp].next.get(&c) {
                        if dest == q {
                            self.states[pp].next.insert(c, clone);
                            pp_opt = self.states[pp].link;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        } else {
            // No state had a transition; link to root.
            self.states[cur].link = Some(0);
        }

        self.last = cur;
    }

    /// Build the automaton from the whole input string.
    pub fn build(s: &str) -> Self {
        let mut sam = Self::new();
        for ch in s.chars() {
            sam.extend(ch);
        }
        sam
    }

    /// Check whether `substr` occurs as a substring of the original string.
    pub fn contains(&self, substr: &str) -> bool {
        if substr.is_empty() {
            return true;
        }
        let mut v = 0usize; // start at root
        for ch in substr.chars() {
            match self.states[v].next.get(&ch) {
                Some(&next_state) => v = next_state,
                None => return false,
            }
        }
        true
    }
}