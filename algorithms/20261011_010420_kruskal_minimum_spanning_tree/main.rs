mod core;

use core::{Edge, kruskal};

fn main() {
    // Simple sanity check executed at runtime
    let vertices = 4usize;
    let edges = vec![
        Edge { u: 0, v: 1, weight: 1 },
        Edge { u: 0, v: 2, weight: 4 },
        Edge { u: 1, v: 2, weight: 2 },
        Edge { u: 1, v: 3, weight: 6 },
        Edge { u: 2, v: 3, weight: 3 },
    ];
    let (mst, total) = kruskal(vertices, edges);
    assert_eq!(mst.len(), vertices - 1);
    assert_eq!(total, 6); // edges 0‑1 (1), 1‑2 (2), 2‑3 (3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_triangle() {
        // Graph: 0-1 (1), 1-2 (2), 0-2 (3)
        let vertices = 3usize;
        let edges = vec![
            Edge { u: 0, v: 1, weight: 1 },
            Edge { u: 1, v: 2, weight: 2 },
            Edge { u: 0, v: 2, weight: 3 },
        ];
        let (mst, total) = kruskal(vertices, edges);
        assert_eq!(mst.len(), vertices - 1);
        assert_eq!(total, 3); // 1 + 2
        // Ensure the selected edges are the two smallest
        let mut selected_weights: Vec<u32> = mst.iter().map(|e| e.weight).collect();
        selected_weights.sort();
        assert_eq!(selected_weights, vec![1, 2]);
    }

    #[test]
    fn test_disconnected_graph() {
        // Two components: {0,1} and {2,3}
        let vertices = 4usize;
        let edges = vec![
            Edge { u: 0, v: 1, weight: 5 },
            Edge { u: 2, v: 3, weight: 7 },
        ];
        let (mst, total) = kruskal(vertices, edges);
        // Forest contains exactly the two edges (no cycles possible)
        assert_eq!(mst.len(), 2);
        assert_eq!(total, 12);
    }

    #[test]
    fn test_cycle_heavy_edge() {
        // Square with a heavy diagonal
        let vertices = 4usize;
        let edges = vec![
            Edge { u: 0, v: 1, weight: 1 },
            Edge { u: 1, v: 2, weight: 1 },
            Edge { u: 2, v: 3, weight: 1 },
            Edge { u: 3, v: 0, weight: 1 },
            Edge { u: 0, v: 2, weight: 10 }, // heavy diagonal
        ];
        let (mst, total) = kruskal(vertices, edges);
        assert_eq!(mst.len(), vertices - 1);
        assert_eq!(total, 3); // three edges of weight 1
        // Ensure the heavy diagonal is not selected
        assert!(!mst.iter().any(|e| e.u == 0 && e.v == 2));
    }

    #[test]
    fn test_multiple_equal_weights() {
        // Complete graph K3 with equal weights
        let vertices = 3usize;
        let edges = vec![
            Edge { u: 0, v: 1, weight: 5 },
            Edge { u: 1, v: 2, weight: 5 },
            Edge { u: 0, v: 2, weight: 5 },
        ];
        let (mst, total) = kruskal(vertices, edges);
        assert_eq!(mst.len(), vertices - 1);
        assert_eq!(total, 10);
        // Any two edges form a valid MST; check that no duplicate vertices in a cycle
        let mut seen = std::collections::HashSet::new();
        for e in mst.iter() {
            assert!(seen.insert(e.u));
            assert!(seen.insert(e.v));
        }
    }

    #[test]
    fn test_large_random() {
        // Deterministic pseudo‑random edges for reproducibility
        let vertices = 6usize;
        let mut edges = Vec::new();
        let weights = [
            (0, 1, 3),
            (0, 2, 1),
            (0, 3, 6),
            (0, 4, 5),
            (0, 5, 2),
            (1, 2, 4),
            (1, 3, 2),
            (1, 4, 7),
            (1, 5, 3),
            (2, 3, 5),
            (2, 4, 8),
            (2, 5, 4),
            (3, 4, 9),
            (3, 5, 7),
            (4, 5, 1),
        ];
        for &(u, v, w) in weights.iter() {
            edges.push(Edge { u, v, weight: w });
        }
        let (mst, total) = kruskal(vertices, edges);
        assert_eq!(mst.len(), vertices - 1);
        // Manually compute expected total weight:
        // Selected edges: (4,5,1), (0,2,1), (0,5,2), (1,3,2), (0,1,3) => total 9
        assert_eq!(total, 9);
    }
}