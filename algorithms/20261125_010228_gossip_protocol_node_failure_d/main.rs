use crate::engine::engine::Engine;
use crate::types::types::NodeStatus;

fn main() {
    // Configuration
    let num_nodes = 5usize;
    let suspect_thresh = 2u64;
    let dead_thresh = 4u64;

    // Initialise engine
    let mut eng = Engine::new(num_nodes, suspect_thresh, dead_thresh);

    // Run a few healthy rounds
    for _ in 0..3 {
        eng.step();
    }

    // Verify all nodes are still alive
    for status in eng.statuses() {
        assert_eq!(status, NodeStatus::Alive);
    }

    // Fail node 2
    eng.fail_node(2);

    // Run enough rounds for others to suspect and then declare dead
    for _ in 0..6 {
        eng.step();
    }

    // After 6 additional rounds (total heartbeat difference >= dead_thresh),
    // every operational node should consider node 2 dead.
    for observer in 0..num_nodes {
        if observer == 2 {
            // The failed node itself does not change status (it is not operational)
            continue;
        }
        assert!(
            eng.perceives_dead(observer, 2),
            "Node {} should perceive node 2 as dead",
            observer
        );
        // The observer's own status must still be Alive
        assert_eq!(eng.nodes[observer].status, NodeStatus::Alive);
    }

    // The failed node's status remains as whatever it was before failure (Alive)
    // because it never updates its view. This is acceptable for this deterministic model.
    // Ensure the engine's internal dead detection aligns with expectations.
    println!("All assertions passed. Gossip failure detector behaves as expected.");
}

// Ensure the modules are compiled as part of the crate
mod types;
mod engine;