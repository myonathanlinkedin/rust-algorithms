use std::collections::HashMap;

/// Represents a single node in the gossip failure detector.
#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    /// Unique identifier of the node.
    pub id: usize,
    /// Monotonically increasing heartbeat counter.
    pub heartbeat: u64,
    /// Known heartbeats of other nodes (including self).
    pub view: HashMap<usize, u64>,
    /// Maximum allowed staleness before a node is considered failed.
    pub timeout: u64,
    /// Whether the node is operational. A failed node does not tick or gossip.
    pub alive: bool,

}

impl Node {
    /// Create a new node with the given identifier and timeout.
    pub fn new(id: usize, timeout: u64) -> Self {
        let mut view = HashMap::new();
        view.insert(id, 0);
        Node {
            id,
            heartbeat: 0,
            view,
            timeout,
            alive: true,
        }
    }

    /// Increment the node's own heartbeat if it is alive.
    pub fn tick(&mut self) {
        if self.alive {
            self.heartbeat = self.heartbeat.wrapping_add(1);
            self.view.insert(self.id, self.heartbeat);
        }
    }

    /// Merge an incoming view into the node's own view.
    /// The larger heartbeat for each node wins.
    pub fn receive_gossip(&mut self, other_view: &HashMap<usize, u64>) {
        if !self.alive {
            return;
        }
        for (&node_id, &hb) in other_view.iter() {
            let entry = self.view.entry(node_id).or_insert(0);
            if hb > *entry {
                *entry = hb;
            }
        }
    }

    /// Return a list of node identifiers that are considered failed
    /// according to the node's current view.
    pub fn detect_failed(&self) -> Vec<usize> {
        let mut failed = Vec::new();
        for (&node_id, &hb) in self.view.iter() {
            if node_id == self.id {
                continue;
            }
            if self.heartbeat.saturating_sub(hb) > self.timeout {
                failed.push(node_id);
            }
        }
        failed
    }
}

/// Simple deterministic pseudo‑random number generator (LCG).
#[derive(Debug, Clone, PartialEq)]
pub struct SimpleRng {
    pub state: u64,

}

impl SimpleRng {
    fn new(seed: u64) -> Self {
        SimpleRng { state: seed }
    }

    /// Return a pseudo‑random `usize`.
    fn next_usize(&mut self) -> usize {
        // Constants from Numerical Recipes
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1);
        // Use the high bits for better distribution
        (self.state >> 33) as usize
    }
}

/// The gossip network containing all nodes.
#[derive(Debug, Clone, PartialEq)]
pub struct GossipNetwork {
    /// All nodes indexed by their identifier.
    pub nodes: Vec<Node>,
    /// Deterministic RNG used for peer selection.
    pub rng: SimpleRng,

}

impl GossipNetwork {
    /// Construct a network with `num_nodes` nodes, each using the same `timeout`.
    pub fn new(num_nodes: usize, timeout: u64) -> Self {
        let mut nodes = Vec::with_capacity(num_nodes);
        for id in 0..num_nodes {
            nodes.push(Node::new(id, timeout));
        }
        GossipNetwork {
            nodes,
            rng: SimpleRng::new(0xdead_beef_cafe_babe),
        }
    }

    /// Retrieve an immutable reference to a node by id.
    pub fn get_node(&self, id: usize) -> Option<&Node> {
        self.nodes.iter().find(|n| n.id == id)
    }

    /// Retrieve a mutable reference to a node by id.
    fn get_node_mut(&mut self, id: usize) -> Option<&mut Node> {
        self.nodes.iter_mut().find(|n| n.id == id)
    }

    /// Simulate a single logical time step:
    /// 1. All alive nodes tick.
    /// 2. Each alive node selects a random alive peer and gossips its view.
    pub fn step(&mut self) {
        // Phase 1: tick
        for node in self.nodes.iter_mut() {
            node.tick();
        }

        // Phase 2: gossip
        // Collect snapshots of views to avoid mutable‑borrow conflicts.
        let snapshots: Vec<(usize, HashMap<usize, u64>)> = self
            .nodes
            .iter()
            .filter(|n| n.alive)
            .map(|n| (n.id, n.view.clone()))
            .collect();

        for (src_id, src_view) in snapshots {
            // Choose a random alive peer different from src_id.
            let peer_id_opt = self.select_random_peer(src_id);
            if let Some(peer_id) = peer_id_opt {
                if let Some(peer) = self.get_node_mut(peer_id) {
                    peer.receive_gossip(&src_view);
                }
            }
        }
    }

    /// Run `steps` logical steps.
    pub fn run(&mut self, steps: usize) {
        for _ in 0..steps {
            self.step();
        }
    }

    /// Mark a node as failed (dead). It will stop ticking and gossiping.
    pub fn fail_node(&mut self, id: usize) {
        if let Some(node) = self.get_node_mut(id) {
            node.alive = false;
        }
    }

    /// Helper to select a random alive peer different from `exclude_id`.
    fn select_random_peer(&mut self, exclude_id: usize) -> Option<usize> {
        let alive_ids: Vec<usize> = self
            .nodes
            .iter()
            .filter(|n| n.alive && n.id != exclude_id)
            .map(|n| n.id)
            .collect();
        if alive_ids.is_empty() {
            return None;
        }
        let idx = self.rng.next_usize() % alive_ids.len();
        Some(alive_ids[idx])
    }
}