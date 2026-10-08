mod types;
mod engine;

use types::SubmodularOracle;
use engine::minimize;
use std::cmp::Ordering;

/// A concrete symmetric submodular function: the cut value of an undirected weighted graph.
///
/// For a set S ⊆ V, the function returns the total weight of edges with exactly one endpoint in S.
#[derive(Debug)]
struct GraphCut {
    n: usize,
    edges: Vec<(usize, usize, i32)>,
}

impl SubmodularOracle for GraphCut {
    fn compare(&self, a: &[usize], b: &[usize]) -> Ordering {
        self.value(a).cmp(&self.value(b))
    }

    fn value(&self, set: &[usize]) -> i32 {
        let mut in_set = vec![false; self.n];
        for &v in set {
            in_set[v] = true;
        }
        let mut sum = 0;
        for &(u, v, w) in &self.edges {
            if in_set[u] != in_set[v] {
                sum += w;
            }
        }
        sum
    }
}

/// A handcrafted symmetric submodular function where the empty and full sets are deliberately
/// penalised, forcing a non‑trivial minimiser.
///
/// f(S) = |S|·(n‑|S|) for 0 < |S| < n, and a large constant otherwise.
#[derive(Debug)]
struct PenalisedCardinality {
    n: usize,
}

impl SubmodularOracle for PenalisedCardinality {
    fn compare(&self, a: &[usize], b: &[usize]) -> Ordering {
        self.value(a).cmp(&self.value(b))
    }

    fn value(&self, set: &[usize]) -> i32 {
        let sz = set.len() as i32;
        let n = self.n as i32;
        if sz == 0 || sz == n {
            100 // high penalty for trivial sets
        } else {
            sz * (n - sz) // symmetric submodular term
        }
    }
}

fn main() {
    // ---- Test 1: Simple triangle graph ----
    // All edges have weight 1. The global minimum of the cut function is the empty set (value 0).
    let n1 = 3;
    let edges1 = vec![(0, 1, 1), (1, 2, 1), (0, 2, 1)];
    let oracle1 = GraphCut { n: n1, edges: edges1 };
    let min_set1 = minimize(n1, &oracle1);
    assert_eq!(min_set1.len(), 0, "Triangle graph should yield empty minimiser");

    // ---- Test 2: Weighted square graph ----
    // Edge weights are chosen so that any single‑vertex cut has value 11, while the empty set is optimal.
    let n2 = 4;
    let edges2 = vec![
        (0, 1, 1),
        (1, 2, 10),
        (2, 3, 1),
        (0, 3, 10),
    ];
    let oracle2 = GraphCut { n: n2, edges: edges2 };
    let min_set2 = minimize(n2, &oracle2);
    assert_eq!(min_set2.len(), 0, "Weighted square should also yield empty minimiser");

    // ---- Test 3: Penalised cardinality function ----
    // Empty and full sets are penalised; the minimum occurs at any set of size 2.
    let penalised = PenalisedCardinality { n: 4 };
    let min_set3 = minimize(4, &penalised);
    assert_eq!(min_set3.len(), 2, "Penalised cardinality should return a 2‑element set");
    // Verify symmetry: the complement also has size 2 and yields the same value.
    let complement: Vec<usize> = (0..4).filter(|v| !min_set3.contains(v)).collect();
    assert_eq!(complement.len(), 2);
    assert_eq!(
        penalised.compare(&min_set3, &complement),
        Ordering::Equal,
        "Complement should have equal value for symmetric function"
    );
}