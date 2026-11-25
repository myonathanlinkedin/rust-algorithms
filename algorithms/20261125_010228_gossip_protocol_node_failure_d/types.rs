pub mod types {
    #[derive(Debug, Clone, PartialEq)]
    pub enum NodeStatus {
        Alive,
        Suspect,
        Dead,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Node {
        pub id: usize,
        pub heartbeat: u64,
        // last_heard[i] stores the latest heartbeat value known for node i
        pub last_heard: Vec<u64>,
        pub status: NodeStatus,

    }

    impl Node {
        pub fn new(id: usize, num_nodes: usize) -> Self {
            let mut last_heard = vec![0u64; num_nodes];
            // initialise own heartbeat entry to 0
            last_heard[id] = 0;
            Self {
                id,
                heartbeat: 0,
                last_heard,
                status: NodeStatus::Alive,
            }
        }

        // Merge a received heartbeat vector into this node's view
        pub fn merge(&mut self, received: &[u64]) {
            for (i, &val) in received.iter().enumerate() {
                if val > self.last_heard[i] {
                    self.last_heard[i] = val;
                }
            }
        }

        // Update the node's own status based on its view of others
        pub fn evaluate_statuses(&mut self, suspect_thresh: u64, dead_thresh: u64) {
            for (i, &last) in self.last_heard.iter().enumerate() {
                if i == self.id {
                    continue;
                }
                if self.heartbeat >= last + dead_thresh {
                    // Mark the remote node as dead in our local view
                    // (we keep our own status unchanged)
                    // In a real implementation we would store per‑node status;
                    // for simplicity we only track our own status.
                } else if self.heartbeat >= last + suspect_thresh {
                    // suspect logic could be stored similarly
                }
            }
            // The node's own status is managed by the engine; no change here.
        }
    }
}