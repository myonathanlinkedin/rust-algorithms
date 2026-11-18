mod core;
use core::{GossipNetwork, Node};

fn main() {
    // Simple demonstration: 5 nodes, timeout = 3, run 10 steps.
    let mut net = GossipNetwork::new(5, 3);
    net.run(10);

    // After enough steps, all alive nodes should have the same view.
    let reference_view = net.get_node(0).unwrap().view.clone();
    for node in net.nodes.iter() {
        assert_eq!(node.view, reference_view);
    }

    // Simulate a failure and ensure detection.
    net.fail_node(2);
    net.run(5);
    let detector = net.get_node(0).unwrap();
    assert!(detector.detect_failed().contains(&2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        let net = GossipNetwork::new(3, 5);
        for id in 0..3 {
            let node = net.get_node(id).expect("node exists");
            assert_eq!(node.id, id);
            assert_eq!(node.heartbeat, 0);
            assert_eq!(node.view.get(&id), Some(&0));
            assert!(node.alive);
        }
    }

    #[test]
    fn test_gossip_propagation() {
        let mut net = GossipNetwork::new(4, 4);
        net.run(8);
        // After several steps, every node should know the max heartbeat of all nodes.
        let mut max_hb = 0;
        for node in net.nodes.iter() {
            if node.heartbeat > max_hb {
                max_hb = node.heartbeat;
            }
        }
        for node in net.nodes.iter() {
            for (&other_id, &hb) in node.view.iter() {
                assert_eq!(hb, max_hb, "node {} view of {} mismatched", node.id, other_id);
            }
        }
    }

    #[test]
    fn test_failure_detection() {
        let mut net = GossipNetwork::new(5, 2);
        // Run a few steps with all nodes alive.
        net.run(3);
        // Fail node 3.
        net.fail_node(3);
        // Continue running; node 3 stops ticking and gossiping.
        net.run(5);
        // All other nodes should now consider node 3 failed.
        for node in net.nodes.iter().filter(|n| n.id != 3) {
            let failed = node.detect_failed();
            assert!(failed.contains(&3), "node {} did not detect failure", node.id);
        }
    }

    #[test]
    fn test_no_false_positives() {
        let mut net = GossipNetwork::new(3, 10);
        net.run(5);
        // No node should be marked as failed because timeout is large.
        for node in net.nodes.iter() {
            assert!(node.detect_failed().is_empty());
        }
    }

    #[test]
    fn test_select_random_peer_edge_cases() {
        let mut net = GossipNetwork::new(1, 1);
        // With a single node, there is no peer to select.
        assert_eq!(net.select_random_peer(0), None);
        // Add a second node and ensure selection works.
        net.nodes.push(Node::new(1, 1));
        // Force deterministic RNG state for reproducibility.
        net.rng = core::SimpleRng::new(42);
        let peer = net.select_random_peer(0);
        assert_eq!(peer, Some(1));
    }
}