mod types;
mod engine;

use types::Graph;
use engine::tarjan_scc;

fn main() {
    // ---------- Test 1: Empty graph ----------
    let g0 = Graph::new(0);
    let scc0 = tarjan_scc(&g0);
    assert!(scc0.is_empty(), "Empty graph should yield no SCCs");

    // ---------- Test 2: Single vertex, no edges ----------
    let g1 = Graph::new(1);
    let scc1 = tarjan_scc(&g1);
    assert_eq!(scc1, vec![vec![0]], "Single vertex should be its own SCC");

    // ---------- Test 3: Two vertices, one directed edge ----------
    let mut g2 = Graph::new(2);
    g2.add_edge(0, 1);
    let scc2 = tarjan_scc(&g2);
    // Expect two separate SCCs.
    assert_eq!(scc2.len(), 2);
    let mut sorted2: Vec<Vec<usize>> = scc2
        .into_iter()
        .map(|mut c| {
            c.sort_unstable();
            c
        })
        .collect();
    sorted2.sort_unstable_by_key(|c| c[0]);
    assert_eq!(sorted2, vec![vec![0], vec![1]]);

    // ---------- Test 4: Simple cycle of three vertices ----------
    let mut g3 = Graph::new(3);
    g3.add_edge(0, 1);
    g3.add_edge(1, 2);
    g3.add_edge(2, 0);
    let scc3 = tarjan_scc(&g3);
    assert_eq!(scc3.len(), 1);
    let mut comp = scc3[0].clone();
    comp.sort_unstable();
    assert_eq!(comp, vec![0, 1, 2]);

    // ---------- Test 5: Two SCCs with an inter‑SCC edge ----------
    // SCC A: 0 <-> 1, SCC B: 2 <-> 3, edge from A to B.
    let mut g4 = Graph::new(4);
    g4.add_edge(0, 1);
    g4.add_edge(1, 0);
    g4.add_edge(2, 3);
    g4.add_edge(3, 2);
    g4.add_edge(1, 2); // A → B
    let scc4 = tarjan_scc(&g4);
    assert_eq!(scc4.len(), 2);
    let mut comps: Vec<Vec<usize>> = scc4
        .into_iter()
        .map(|mut c| {
            c.sort_unstable();
            c
        })
        .collect();
    comps.sort_unstable_by_key(|c| c[0]);
    assert_eq!(comps, vec![vec![0, 1], vec![2, 3]]);

    // ---------- Test 6: Self‑loop ----------
    let mut g5 = Graph::new(2);
    g5.add_edge(0, 0); // self‑loop makes 0 its own SCC
    g5.add_edge(0, 1);
    let scc5 = tarjan_scc(&g5);
    assert_eq!(scc5.len(), 2);
    let mut comps5: Vec<Vec<usize>> = scc5
        .into_iter()
        .map(|mut c| {
            c.sort_unstable();
            c
        })
        .collect();
    comps5.sort_unstable_by_key(|c| c[0]);
    assert_eq!(comps5, vec![vec![0], vec![1]]);

    // All assertions passed.
    println!("All Tarjan SCC tests passed.");
}