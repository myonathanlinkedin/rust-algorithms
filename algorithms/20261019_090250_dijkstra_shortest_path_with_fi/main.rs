mod core;
use core::{dijkstra, Graph};

fn main() {
    // Simple sanity check
    let mut g = Graph::new(3);
    g.add_undirected(0, 1, 5);
    g.add_undirected(1, 2, 7);
    let d = dijkstra(&g, 0);
    assert_eq!(d[0], 0);
    assert_eq!(d[1], 5);
    assert_eq!(d[2], 12);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic_graph() {
        let mut g = Graph::new(5);
        g.add_undirected(0, 1, 10);
        g.add_undirected(0, 2, 3);
        g.add_undirected(1, 2, 1);
        g.add_undirected(1, 3, 2);
        g.add_undirected(2, 3, 8);
        g.add_undirected(2, 4, 2);
        g.add_undirected(3, 4, 7);
        g.add_undirected(4, 3, 9);
        let d = dijkstra(&g, 0);
        let expected = vec![0, 4, 3, 12, 5];
        assert_eq!(d, expected);
    }

    #[test]
    fn test_disconnected() {
        let mut g = Graph::new(3);
        g.add_edge(0, 1, 5);
        let d = dijkstra(&g, 0);
        assert_eq!(d[0], 0);
        assert_eq!(d[1], 5);
        assert_eq!(d[2], u64::MAX);
    }

    #[test]
    fn test_single_node() {
        let g = Graph::new(1);
        let d = dijkstra(&g, 0);
        assert_eq!(d, vec![0]);
    }

    #[test]
    fn test_zero_weight_edges() {
        let mut g = Graph::new(4);
        g.add_undirected(0, 1, 0);
        g.add_undirected(1, 2, 0);
        g.add_undirected(2, 3, 5);
        let d = dijkstra(&g, 0);
        let expected = vec![0, 0, 0, 5];
        assert_eq!(d, expected);
    }
}