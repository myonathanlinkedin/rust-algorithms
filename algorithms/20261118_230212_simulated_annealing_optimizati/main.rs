mod core;
use core::{Graph, SimpleRng, simulated_annealing};

fn main() {
    // Simple demonstration on a 4‑node square graph
    let mut g = Graph::new(4);
    g.add_edge(0, 1, 1.0);
    g.add_edge(1, 2, 1.0);
    g.add_edge(2, 3, 1.0);
    g.add_edge(3, 0, 1.0);
    // Diagonal edges to make the optimum cut weight 4
    g.add_edge(0, 2, 0.0);
    g.add_edge(1, 3, 0.0);

    let mut rng = SimpleRng::new(42);
    let (assignment, score) = simulated_annealing(
        &g,
        &mut rng,
        10.0,      // initial temperature
        0.95,      // cooling factor
        100,       // iterations per temperature
        1e-3,      // minimum temperature
    );

    // The optimal Max‑Cut for this graph is 4.0
    assert!((score - 4.0).abs() < 1e-6, "Score {} differs from optimum", score);
    // Ensure assignment is a valid cut (no out‑of‑range indices)
    assert_eq!(assignment.len(), g.vertices());
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delta_flip_consistency() {
        let mut g = Graph::new(3);
        g.add_edge(0, 1, 2.0);
        g.add_edge(1, 2, 3.0);
        g.add_edge(0, 2, 1.0);
        let assignment = vec![true, false, true];
        let delta = g.delta_flip(&assignment, 1);
        // Manually compute: flipping vertex 1 changes edges (0,1) and (1,2)
        // Edge (0,1): before crossing (true vs false) -> after internal => -2.0
        // Edge (1,2): before internal (false vs true) -> after crossing => +3.0
        // Edge (1,2) weight 3, Edge (0,1) weight 2, total delta = +1.0
        assert!((delta - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_small_square_max_cut() {
        let mut g = Graph::new(4);
        g.add_edge(0, 1, 1.0);
        g.add_edge(1, 2, 1.0);
        g.add_edge(2, 3, 1.0);
        g.add_edge(3, 0, 1.0);
        // No diagonals; max cut weight is 4.0 (alternating assignment)
        let mut rng = SimpleRng::new(12345);
        let (assignment, score) = simulated_annealing(
            &g,
            &mut rng,
            5.0,
            0.9,
            200,
            1e-4,
        );
        assert!((score - 4.0).abs() < 1e-6);
        // Verify that the assignment indeed yields that score
        let recomputed = g.cut_score(&assignment);
        assert!((recomputed - score).abs() < 1e-9);
    }

    #[test]
    fn test_triangle_graph() {
        // Complete graph K3 with unit weights; max cut = 2.0
        let mut g = Graph::new(3);
        g.add_edge(0, 1, 1.0);
        g.add_edge(1, 2, 1.0);
        g.add_edge(0, 2, 1.0);
        let mut rng = SimpleRng::new(777);
        let (assignment, score) = simulated_annealing(
            &g,
            &mut rng,
            3.0,
            0.85,
            150,
            1e-5,
        );
        assert!((score - 2.0).abs() < 1e-6);
    }

    #[test]
    fn deterministic_behavior_with_fixed_seed() {
        // Verify that two runs with the same seed produce identical results
        let mut g = Graph::new(5);
        g.add_edge(0, 1, 2.0);
        g.add_edge(0, 2, 1.5);
        g.add_edge(1, 3, 2.5);
        g.add_edge(2, 4, 1.0);
        g.add_edge(3, 4, 3.0);
        let mut rng1 = SimpleRng::new(999);
        let mut rng2 = SimpleRng::new(999);
        let (assign1, score1) = simulated_annealing(&g, &mut rng1, 8.0, 0.92, 120, 1e-4);
        let (assign2, score2) = simulated_annealing(&g, &mut rng2, 8.0, 0.92, 120, 1e-4);
        assert_eq!(assign1, assign2);
        assert!((score1 - score2).abs() < 1e-9);
    }
}