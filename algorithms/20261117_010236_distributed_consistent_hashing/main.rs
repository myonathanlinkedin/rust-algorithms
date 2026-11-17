mod core;
use core::ConsistentHashRouter;

fn main() {
    // Basic sanity checks executed at program start.
    let mut router = ConsistentHashRouter::new(3);
    router.add_node("NodeA".to_string());
    router.add_node("NodeB".to_string());

    // Deterministic mapping: the same key always yields the same node.
    let key = "my-test-key";
    let node1 = router.get_node(&key).expect("router should not be empty");
    let node2 = router.get_node(&key).expect("router should not be empty");
    assert_eq!(node1, node2, "consistent hashing must be deterministic");

    // Removing a node should not panic and should update the mapping.
    router.remove_node(&"NodeA".to_string());
    let node_after_removal = router.get_node(&key).expect("router still has NodeB");
    assert_eq!(node_after_removal, "NodeB".to_string());

    // Empty router edge case.
    let mut empty_router: ConsistentHashRouter<String> = ConsistentHashRouter::new(5);
    assert!(empty_router.get_node(&"any").is_none(), "empty router returns None");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_add_and_lookup() {
        let mut router = ConsistentHashRouter::new(4);
        router.add_node("A".to_string());
        router.add_node("B".to_string());
        router.add_node("C".to_string());

        // All nodes should be present.
        assert_eq!(router.node_count(), 3);
        assert_eq!(router.virtual_node_count(), 12);

        // Keys should map to one of the added nodes.
        let keys = vec!["alpha", "beta", "gamma", "delta", "epsilon"];
        for k in keys {
            let node = router.get_node(&k).expect("router not empty");
            assert!(["A", "B", "C"].contains(&node.as_str()));
        }
    }

    #[test]
    fn test_removal_updates_ring() {
        let mut router = ConsistentHashRouter::new(2);
        router.add_node(1u32);
        router.add_node(2u32);
        router.add_node(3u32);

        let key = "some-key";
        let assigned_before = router.get_node(&key).unwrap();

        router.remove_node(&assigned_before);
        // After removal, the same key must map to a different node.
        let assigned_after = router.get_node(&key).unwrap();
        assert_ne!(assigned_before, assigned_after);
        assert_eq!(router.node_count(), 2);
        assert_eq!(router.virtual_node_count(), 4);
    }

    #[test]
    fn test_idempotent_add_and_remove() {
        let mut router = ConsistentHashRouter::new(1);
        router.add_node("X".to_string());
        router.add_node("X".to_string()); // duplicate add should be ignored
        assert_eq!(router.node_count(), 1);
        assert_eq!(router.virtual_node_count(), 1);

        router.remove_node(&"Y".to_string()); // removing non‑existent node is safe
        assert_eq!(router.node_count(), 1);
        router.remove_node(&"X".to_string());
        assert_eq!(router.node_count(), 0);
        assert!(router.get_node(&"anything").is_none());
    }

    #[test]
    fn test_deterministic_hashing_across_instances() {
        // Two routers with identical configuration and nodes must produce identical mappings.
        let mut r1 = ConsistentHashRouter::new(3);
        let mut r2 = ConsistentHashRouter::new(3);
        for node in &["A", "B", "C"] {
            r1.add_node(node.to_string());
            r2.add_node(node.to_string());
        }

        let keys = vec!["k1", "k2", "k3", "k4"];
        for k in keys {
            let n1 = r1.get_node(&k).unwrap();
            let n2 = r2.get_node(&k).unwrap();
            assert_eq!(n1, n2, "both routers must map key '{}' to the same node", k);
        }
    }

    #[test]
    fn test_empty_router_behavior() {
        let router: ConsistentHashRouter<i32> = ConsistentHashRouter::new(5);
        assert!(router.get_node(&42).is_none());
        assert_eq!(router.node_count(), 0);
        assert_eq!(router.virtual_node_count(), 0);
    }
}