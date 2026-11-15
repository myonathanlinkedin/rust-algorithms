mod core;
use core::core::ChronoGraph;

fn main() {
    // Simple demonstration that also serves as a runtime sanity check.
    let mut g = ChronoGraph::new(6);
    g.add_edge(0, 1);
    g.add_edge(0, 2);
    g.add_edge(1, 3);
    g.add_edge(2, 3);
    g.add_edge(3, 4);
    g.add_edge(4, 5);

    let order = g.topological_order().expect("graph should be acyclic");
    // Verify that the order respects the edges (runtime assert, not test).
    assert!(order.iter().position(|&x| x == 0).unwrap()
        < order.iter().position(|&x| x == 1).unwrap());
    assert!(order.iter().position(|&x| x == 0).unwrap()
        < order.iter().position(|&x| x == 2).unwrap());
    assert!(order.iter().position(|&x| x == 1).unwrap()
        < order.iter().position(|&x| x == 3).unwrap());
    assert!(order.iter().position(|&x| x == 2).unwrap()
        < order.iter().position(|&x| x == 3).unwrap());
    assert!(order.iter().position(|&x| x == 3).unwrap()
        < order.iter().position(|&x| x == 4).unwrap());
    assert!(order.iter().position(|&x| x == 4).unwrap()
        < order.iter().position(|&x| x == 5).unwrap());

    let times = g.earliest_start_times().expect("graph should be acyclic");
    // Expected longest distances: 0,1,1,2,3,4
    assert_eq!(times, vec![0, 1, 1, 2, 3, 4]);

    println!("Topological order: {:?}", order);
    println!("Earliest start times: {:?}", times);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topological_order_simple() {
        let mut g = ChronoGraph::new(4);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(1, 3);
        g.add_edge(2, 3);

        let order = g.topological_order().unwrap();

        // Verify ordering constraints
        let pos = |v| order.iter().position(|&x| x == v).unwrap();
        assert!(pos(0) < pos(1));
        assert!(pos(0) < pos(2));
        assert!(pos(1) < pos(3));
        assert!(pos(2) < pos(3));
    }

    #[test]
    fn test_cycle_detection() {
        let mut g = ChronoGraph::new(3);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 0);

        assert!(g.topological_order().is_err());
        assert!(g.earliest_start_times().is_err());
    }

    #[test]
    fn test_earliest_start_times() {
        let mut g = ChronoGraph::new(5);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(1, 3);
        g.add_edge(2, 3);
        g.add_edge(3, 4);

        let times = g.earliest_start_times().unwrap();
        // Expected longest distances from source 0
        assert_eq!(times, vec![0, 1, 1, 2, 3]);
    }

    #[test]
    fn test_empty_graph() {
        let g = ChronoGraph::new(0);
        let order = g.topological_order().unwrap();
        let times = g.earliest_start_times().unwrap();
        assert!(order.is_empty());
        assert!(times.is_empty());
    }

    #[test]
    fn test_disconnected_graph() {
        let mut g = ChronoGraph::new(4);
        // No edges; each vertex is independent.
        let order = g.topological_order().unwrap();
        let times = g.earliest_start_times().unwrap();
        assert_eq!(order.len(), 4);
        assert_eq!(times, vec![0, 0, 0, 0]);
    }
}