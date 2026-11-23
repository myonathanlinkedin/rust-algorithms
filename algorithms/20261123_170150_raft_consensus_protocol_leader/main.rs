mod types;
mod engine;

use crate::types::{NodeId, Role, Message, VoteRequest};
use crate::engine::RaftNode;
use std::collections::HashMap;

fn main() {
    // Deterministic simulation with three nodes.
    let node_ids: Vec<NodeId> = vec![1, 2, 3];
    let election_timeout = 3usize; // ticks before a follower starts an election

    // Build the cluster.
    let mut nodes: HashMap<NodeId, RaftNode> = HashMap::new();
    for &id in &node_ids {
        let peers = node_ids.iter().cloned().filter(|&p| p != id).collect();
        nodes.insert(id, RaftNode::new(id, peers, election_timeout));
    }

    // Run the simulation until a single leader appears or we exceed a safe bound.
    let mut leader_opt: Option<NodeId> = None;
    for _tick in 0..20 {
        // Phase 1: each node ticks and may emit messages.
        let mut outbound: Vec<(NodeId, NodeId, Message)> = Vec::new(); // (to, from, msg)
        for node in nodes.values_mut() {
            let msgs = node.tick();
            for (to, msg) in msgs {
                outbound.push((to, node.id, msg));
            }
        }

        // Phase 2: deliver all messages. Replies are collected for the next iteration.
        let mut next_outbound: Vec<(NodeId, NodeId, Message)> = Vec::new();
        for (to, from, msg) in outbound {
            if let Some(target) = nodes.get_mut(&to) {
                let replies = target.handle_message(from, msg);
                // Replies are sent back to the original sender.
                for (dest, reply) in replies {
                    next_outbound.push((dest, target.id, reply));
                }
            }
        }
        // Replies are processed in the same tick to keep the simulation simple.
        for (to, from, msg) in next_outbound {
            if let Some(target) = nodes.get_mut(&to) {
                let _ = target.handle_message(from, msg);
            }
        }

        // Check if a unique leader exists.
        let leaders: Vec<_> = nodes.values().filter(|n| n.role == Role::Leader).collect();
        if leaders.len() == 1 {
            leader_opt = Some(leaders[0].id);
            break;
        }
    }

    // ---------- Assertions ----------
    assert!(leader_opt.is_some(), "A leader should have been elected");
    let leader_id = leader_opt.unwrap();

    // All nodes must agree on the term and role.
    let leader_term = nodes.get(&leader_id).unwrap().current_term;
    for node in nodes.values() {
        if node.id == leader_id {
            assert_eq!(node.role, Role::Leader);
            assert_eq!(node.current_term, leader_term);
        } else {
            assert_eq!(node.role, Role::Follower);
            assert_eq!(node.current_term, leader_term);
        }
    }

    // Edge case: a node with a higher term forces the current leader to step down.
    let mut outsider = RaftNode::new(99, vec![], election_timeout);
    outsider.current_term = leader_term + 5;
    outsider.role = Role::Candidate;

    let higher_req = VoteRequest {
        term: outsider.current_term,
        candidate_id: outsider.id,
    };
    // Send the higher-term RequestVote to the current leader.
    let _ = nodes
        .get_mut(&leader_id)
        .unwrap()
        .handle_message(outsider.id, Message::RequestVote(higher_req));

    // Leader must have stepped down to follower.
    assert_eq!(
        nodes.get(&leader_id).unwrap().role,
        Role::Follower,
        "Leader should step down on higher term"
    );
    // Outsider remains candidate (no response handling needed for this test).
    assert_eq!(outsider.role, Role::Candidate);
}