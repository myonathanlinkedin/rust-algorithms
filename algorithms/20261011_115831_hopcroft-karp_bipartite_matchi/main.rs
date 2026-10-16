mod types;
mod engine;

use crate::types::{BipartiteGraph, MatchingResult};
use crate::engine::hopcroft_karp;

fn verify_example() {
    // Example graph:
    // Left vertices: 0,1,2
    // Right vertices: 0,1,2,3
    // Edges: 0-0, 0-1, 1-1, 1-2, 2-2, 2-3
    let graph = BipartiteGraph::new(
        3,
        4,
        vec![
            vec![0, 1], // edges from left 0
            vec![1, 2], // edges from left 1
            vec![2, 3], // edges from left 2
        ],
    );

    let result = hopcroft_karp(&graph);
    assert_eq!(result.matching_size, 3);
    // Verify that each left vertex is matched to a distinct right vertex
    let mut used_right = vec![false; graph.n_right];
    for (u, opt_v) in result.pair_u.iter().enumerate() {
        if let Some(v) = opt_v {
            assert!(!used_right[*v], "Right vertex {} matched twice", v);
            used_right[*v] = true;
            assert_eq!(result.pair_v[*v], Some(u));
        } else {
            panic!("Left vertex {} should be matched", u);
        }
    }
}

fn verify_empty_graph() {
    let graph = BipartiteGraph::new(0, 0, vec![]);
    let result = hopcroft_karp(&graph);
    assert_eq!(result.matching_size, 0);
    assert!(result.pair_u.is_empty());
    assert!(result.pair_v.is_empty());
}

fn verify_no_edges() {
    let graph = BipartiteGraph::new(3, 3, vec![vec![], vec![], vec![]]);
    let result = hopcroft_karp(&graph);
    assert_eq!(result.matching_size, 0);
    for opt in result.pair_u.iter() {
        assert!(opt.is_none());
    }
    for opt in result.pair_v.iter() {
        assert!(opt.is_none());
    }
}

fn main() {
    verify_example();
    verify_empty_graph();
    verify_no_edges();

    // Additional sanity check with a complete bipartite graph K_{2,3}
    let graph = BipartiteGraph::new(
        2,
        3,
        vec![
            vec![0, 1, 2],
            vec![0, 1, 2],
        ],
    );
    let result = hopcroft_karp(&graph);
    assert_eq!(result.matching_size, 2);
    // Ensure each left vertex is matched and right vertices are distinct
    let mut matched_right = vec![false; graph.n_right];
    for opt in result.pair_u.iter() {
        if let Some(v) = opt {
            assert!(!matched_right[*v]);
            matched_right[*v] = true;
        }
    }
    println!("All assertions passed. Hopcroft‑Karp implementation is verified.");
}

// Optional test module for `cargo test`
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        verify_example();
    }

    #[test]
    fn test_empty() {
        verify_empty_graph();
    }

    #[test]
    fn test_no_edges() {
        verify_no_edges();
    }

    #[test]
    fn test_complete_bipartite() {
        let graph = BipartiteGraph::new(
            4,
            4,
            vec![
                vec![0, 1, 2, 3],
                vec![0, 1, 2, 3],
                vec![0, 1, 2, 3],
                vec![0, 1, 2, 3],
            ],
        );
        let result = hopcroft_karp(&graph);
        assert_eq!(result.matching_size, 4);
    }
}