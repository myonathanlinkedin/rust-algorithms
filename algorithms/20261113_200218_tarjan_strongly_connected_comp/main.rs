mod types;
mod engine;

use types::Graph;
use engine::tarjan_scc;

fn main() {
    // Example graph:
    // 0 → 1 → 2 → 0   (SCC {0,1,2})
    // 2 → 3
    // 3 → 4 → 5 → 3   (SCC {3,4,5})
    // 5 → 6            (SCC {6})
    let mut g = Graph::new(7);
    g.add_edge(0, 1);
    g.add_edge(1, 2);
    g.add_edge(2, 0);
    g.add_edge(2, 3);
    g.add_edge(3, 4);
    g.add_edge(4, 5);
    g.add_edge(5, 3);
    g.add_edge(5, 6);

    let sccs = tarjan_scc(&g);

    // Expected SCCs (order may vary, but each component is sorted)
    let mut expected = vec![
        vec![0, 1, 2],
        vec![3, 4, 5],
        vec![6],
    ];
    // Sort outer vector for order‑independent comparison
    let mut sccs_sorted = sccs.clone();
    sccs_sorted.sort();
    expected.sort();

    assert_eq!(sccs_sorted, expected, "SCC decomposition did not match expected result");

    // Additional edge‑case tests

    // 1. Empty graph
    let empty = Graph::new(0);
    let empty_scc = tarjan_scc(&empty);
    assert!(empty_scc.is_empty(), "Empty graph should yield no SCCs");

    // 2. Single vertex, no edges
    let single = Graph::new(1);
    let single_scc = tarjan_scc(&single);
    assert_eq!(single_scc, vec![vec![0]], "Single vertex graph should yield one SCC containing the vertex");

    // 3. Linear chain (no cycles)
    let mut chain = Graph::new(4);
    chain.add_edge(0, 1);
    chain.add_edge(1, 2);
    chain.add_edge(2, 3);
    let chain_scc = tarjan_scc(&chain);
    let expected_chain = vec![vec![0], vec![1], vec![2], vec![3]];
    assert_eq!(chain_scc, expected_chain, "Linear chain should produce each vertex as its own SCC");

    // 4. Complete directed graph of 3 vertices (every vertex reachable from every other)
    let mut complete = Graph::new(3);
    for i in 0..3 {
        for j in 0..3 {
            if i != j {
                complete.add_edge(i, j);
            }
        }
    }
    let complete_scc = tarjan_scc(&complete);
    assert_eq!(complete_scc, vec![vec![0, 1, 2]], "Complete digraph should be a single SCC");

    println!("All Tarjan SCC tests passed.");
}