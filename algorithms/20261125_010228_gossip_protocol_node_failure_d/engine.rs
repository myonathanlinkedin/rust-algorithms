pub mod engine {
    use crate::types::types::{Node, NodeStatus};

    pub struct Engine {
        pub nodes: Vec<Node>,
        pub operational: Vec<bool>, // true if the node is still sending heartbeats
        pub suspect_threshold: u64,
        pub dead_threshold: u64,
        pub round: u64,

    }

    impl Engine {
        pub fn new(num_nodes: usize, suspect_threshold: u64, dead_threshold: u64) -> Self {
            let mut nodes = Vec::with_capacity(num_nodes);
            for id in 0..num_nodes {
                nodes.push(Node::new(id, num_nodes));
            }
            Self {
                nodes,
                operational: vec![true; num_nodes],
                suspect_threshold,
                dead_threshold,
                round: 0,
            }
        }

        // Simulate a single gossip round
        pub fn step(&mut self) {
            let n = self.nodes.len();

            // 1. Increment heartbeat for operational nodes
            for (i, node) in self.nodes.iter_mut().enumerate() {
                if self.operational[i] {
                    node.heartbeat += 1;
                    node.last_heard[i] = node.heartbeat;
                }
            }

            // 2. Deterministic gossip: each node i contacts (i + 1) % n
            // Perform exchanges in two phases to avoid mutable‑borrow conflicts
            let mut messages: Vec<Option<Vec<u64>>> = vec![None; n];
            for i in 0..n {
                if self.operational[i] {
                    // Clone the view to send
                    messages[i] = Some(self.nodes[i].last_heard.clone());
                }
            }

            // Apply received messages
            for i in 0..n {
                let peer = (i + 1) % n;
                if let (Some(ref msg), true) = (&messages[i], self.operational[peer]) {
                    // peer receives msg from i
                    self.nodes[peer].merge(msg);
                }
            }

            // 3. Evaluate statuses for each node
            for node in self.nodes.iter_mut() {
                node.evaluate_statuses(self.suspect_threshold, self.dead_threshold);
            }

            // 4. Update per‑node global status (Alive / Suspect / Dead)
            // A node becomes Suspect if any other node hasn't been heard from
            // for >= suspect_threshold rounds, and Dead if >= dead_threshold.
            // For simplicity we compute status based on the node's own view of others.
            for i in 0..n {
                let mut suspect = false;
                let mut dead = false;
                for (j, &last) in self.nodes[i].last_heard.iter().enumerate() {
                    if i == j {
                        continue;
                    }
                    let age = self.nodes[i].heartbeat.saturating_sub(last);
                    if age >= self.dead_threshold {
                        dead = true;
                        break;
                    } else if age >= self.suspect_threshold {
                        suspect = true;
                    }
                }
                self.nodes[i].status = if dead {
                    NodeStatus::Dead
                } else if suspect {
                    NodeStatus::Suspect
                } else {
                    NodeStatus::Alive
                };
            }

            self.round += 1;
        }

        // Mark a node as failed (it stops sending heartbeats and gossip)
        pub fn fail_node(&mut self, id: usize) {
            if id < self.operational.len() {
                self.operational[id] = false;
            }
        }

        // Retrieve the status vector for inspection
        pub fn statuses(&self) -> Vec<NodeStatus> {
            self.nodes.iter().map(|n| n.status.clone()).collect()
        }

        // Helper to check if a particular node perceives another as dead
        pub fn perceives_dead(&self, observer: usize, target: usize) -> bool {
            if observer >= self.nodes.len() || target >= self.nodes.len() {
                return false;
            }
            let age = self.nodes[observer].heartbeat.saturating_sub(self.nodes[observer].last_heard[target]);
            age >= self.dead_threshold
        }
    }
}