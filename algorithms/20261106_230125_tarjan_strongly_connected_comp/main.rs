mod core;
use core::Graph;

fn assert_eq_scc(mut got: Vec<Vec<usize>>, mut expected: Vec<Vec<usize>>) {
    // Both vectors are already sorted by implementation, but ensure sorting for safety
    for comp in &mut got {
        comp.sort_unstable();
    }
    for comp in &mut expected {
        comp.sort_unstable();
    }
    got.sort_by_key(|c| c[0]);
    expected.sort_by_key(|c| c[0]);
    assert_eq!(got, expected);
}

fn main() {
    // Test 1: Empty graph (0 vertices)
    let g0 = Graph::new(0);
    assert_eq!(g0.strongly_connected_components(), Vec::<Vec<usize>>::new());

    // Test 2: Single vertex, no edges
    let g1 = Graph::new(1);
    assert_eq!(g1.strongly_connected_components(), vec![vec![0]]);

    // Test 3: Single vertex with a self‑loop
    let mut g_self = Graph::new(1);
    g_self.add_edge(0, 0);
    assert_eq!(g_self.strongly_connected_components(), vec![vec![0]]);

    // Test 4: Two vertices, one direction (0 → 1)
    let mut g_dir = Graph::new(2);
    g_dir.add_edge(0, 1);
    assert_eq_scc(g_dir.strongly_connected_components(), vec![vec![0], vec![1]]);

    // Test 5: Two vertices, bidirectional (0 ↔ 1) => one SCC
    let mut g_bi = Graph::new(2);
    g_bi.add_edge(0, 1);
    g_bi.add_edge(1, 0);
    assert_eq_scc(g_bi.strongly_connected_components(), vec![vec![0, 1]]);

    // Test 6: Classic example with three SCCs
    // Graph:
    // 0 → 1 → 2 → 0 (cycle)
    // 3 → 4
    // 5 (isolated)
    // 2 → 3, 4 → 5
    let mut g = Graph::new(6);
    g.add_edge(0, 1);
    g.add_edge(1, 2);
    g.add_edge(2, 0);
    g.add_edge(2, 3);
    g.add_edge(3, 4);
    g.add_edge(4, 5);
    let expected = vec![vec![0, 1, 2], vec![3], vec![4], vec![5]];
    assert_eq_scc(g.strongly_connected_components(), expected);

    // Test 7: Larger graph with multiple interleaved SCCs
    // 0→1, 1→2, 2→0 (SCC A)
    // 3→4, 4→5, 5→3 (SCC B)
    // 2→3 (edge from A to B)
    // 5→6 (edge from B to singleton C)
    // 6 (singleton)
    let mut g2 = Graph::new(7);
    g2.add_edge(0, 1);
    g2.add_edge(1, 2);
    g2.add_edge(2, 0);
    g2.add_edge(2, 3);
    g2.add_edge(3, 4);
    g2.add_edge(4, 5);
    g2.add_edge(5, 3);
    g2.add_edge(5, 6);
    let expected2 = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6]];
    assert_eq_scc(g2.strongly_connected_components(), expected2);

    // All assertions passed
    println!("All Tarjan SCC tests passed.");
}